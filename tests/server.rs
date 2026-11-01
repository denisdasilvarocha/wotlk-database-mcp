//! End-to-end tests through a real MCP client and a database built from fixtures.

mod common;

use std::{
    path::{Path, PathBuf},
    sync::OnceLock,
};

use rmcp::{
    ServiceExt,
    model::{CallToolRequestParams, CallToolResult, ErrorCode},
    serde_json::{Value, json},
    service::ServiceError,
};
use wotlk_database_mcp::WotlkServer;

#[expect(clippy::expect_used, reason = "Test fixture setup must fail loudly on invalid input")]
fn fixture_db() -> &'static Path {
    static DB: OnceLock<PathBuf> = OnceLock::new();
    DB.get_or_init(|| {
        let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("server-fixture");
        let inputs = common::write_inputs(&dir, |_| {});
        let db = dir.join("wotlk.sqlite");
        wotlk_db_builder::build(&inputs, &db).expect("build fixture database");
        db
    })
}

#[expect(clippy::expect_used, reason = "Test client setup must fail loudly on invalid input")]
async fn call(tool: &str, args: Value) -> Result<CallToolResult, ServiceError> {
    let (server_io, client_io) = tokio::io::duplex(65536);
    let server = WotlkServer::open(fixture_db()).expect("open fixture database");
    let task = tokio::spawn(async move {
        server
            .serve(server_io)
            .await
            .expect("serve")
            .waiting()
            .await
            .expect("wait for server")
    });
    let client = ().serve(client_io).await.expect("connect client");
    let result = client
        .call_tool(
            CallToolRequestParams::new(tool.to_owned())
                .with_arguments(args.as_object().expect("object arguments").clone()),
        )
        .await;
    client.cancel().await.expect("cancel client");
    task.await.expect("join server");
    result
}

fn text(result: &CallToolResult) -> String {
    result
        .content
        .iter()
        .filter_map(|content| content.as_text())
        .map(|text| text.text.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

#[expect(
    clippy::expect_used,
    reason = "Successful requests must fail the test on transport errors"
)]
async fn success(tool: &str, args: Value) -> String {
    let result = call(tool, args).await.expect("successful MCP request");
    assert_ne!(result.is_error, Some(true), "tool error: {}", text(&result));
    text(&result)
}

#[expect(clippy::expect_used, reason = "Tool errors must fail the test on transport errors")]
async fn tool_error(tool: &str, args: Value, expected: &str) {
    let result = call(tool, args).await.expect("tool error is a result");
    assert_eq!(result.is_error, Some(true), "{}", text(&result));
    assert!(text(&result).contains(expected), "{}", text(&result));
}

async fn invalid(tool: &str, args: Value) {
    let result = call(tool, args).await;
    assert!(
        matches!(&result, Err(ServiceError::McpError(error)) if error.code == ErrorCode::INVALID_PARAMS),
        "{result:?}"
    );
}

fn lines_include(actual: &str, expected: &[&str]) {
    for line in expected {
        assert!(
            actual.lines().any(|actual_line| actual_line == *line),
            "missing {line:?} in {actual:?}"
        );
    }
}

#[tokio::test]
async fn dump_escapes_and_decoded_fields() {
    let search = success("search", json!({"text":"tyrael"})).await;
    assert!(
        search.lines().any(|line| line.starts_with("item 19020 Tyrael's Hilt")),
        "{search}"
    );
    for (id, name) in [
        (19021, r"name=Back\\slash Blade"),
        (19022, "name=O'Reilly's Ring"),
        (19019, "name=Thunderfury, Blessed Blade of the Windseeker"),
    ] {
        lines_include(&success("get", json!({"kind":"item", "id":id})).await, &[name]);
    }
    let item = success("get", json!({"kind":"item", "id":19019, "fields":["flags"]})).await;
    lines_include(
        &item,
        &[
            "flags=CONJURED",
            "quality=LEGENDARY",
            "class=WEAPON",
            "subclass=WEAPON_SWORD",
            "bonding=BIND_WHEN_PICKED_UP",
            "stats=STAMINA +16",
            "icon=INV_Sword_39",
        ],
    );
    lines_include(
        &success("get", json!({"kind":"creature", "id":5001})).await,
        &["npc_flags=TRAINER|TRAINER_CLASS"],
    );
    lines_include(
        &success("get", json!({"kind":"spell", "id":133, "fields":["attributes"]})).await,
        &["attributes=ATTR0_ABILITY"],
    );
}

#[tokio::test]
async fn spells_resolve_descriptions_and_keep_raw_tokens() {
    let spell = success(
        "get",
        json!({"kind":"spell", "id":133, "fields":["description_raw","effect1","effect2"]}),
    )
    .await;
    lines_include(
        &spell,
        &[
            "description=Hurls a fiery ball that causes 14 to 22 Fire damage and an additional 2 Fire damage over 4 sec. Rank 2 deals 31. ${$m1*2} $lpoint:points;",
            "description_raw=Hurls a fiery ball that causes $s1 Fire damage and an additional $o2 Fire damage over $d. Rank 2 deals $143s1. ${$m1*2} $lpoint:points;",
            "school=FIRE",
            "power_type=MANA",
            "mana_cost=30",
            "cast_time=1500",
            "range_max=35",
            "duration=4000",
            "effect1=SCHOOL_DAMAGE value=14-22 target=UNIT_TARGET_ENEMY",
            "effect2=APPLY_AURA PERIODIC_DAMAGE value=1 period=2000 target=UNIT_TARGET_ENEMY",
        ],
    );
    lines_include(
        &success("get", json!({"kind":"spell", "id":2136})).await,
        &["description=Blasts within 10 yards."],
    );
}

#[tokio::test]
async fn offsets_preserve_addresses_and_statuses() {
    let offset = success(
        "get",
        json!({"kind":"offset", "id":"WoWOffsets::Lua::FrameScript::tocfunction"}),
    )
    .await;
    lines_include(
        &offset,
        &[
            "status=not_present",
            "domain=Lua",
            "platform=windows-x86",
            "va=0x0084E1C0",
            "rva=0x0044E1C0",
        ],
    );
    assert!(
        offset
            .lines()
            .any(|line| line.starts_with("comment=") && line.contains("**Not present in build 12340**")),
        "{offset}"
    );
    for (id, status) in [
        ("WoWOffsets::Lua::FrameScript::Execute", "verified"),
        ("WoWOffsets::Unit::CGUnit_C::IsFriendlyTowards", "uncertain"),
        ("WoWOffsets::Globals::Timestamp", "unverified"),
        ("WoWOffsets::Network::CClientConnection::Send", "label_unverified"),
    ] {
        let result = success("get", json!({"kind":"offset", "id":id})).await;
        lines_include(&result, &[&format!("status={status}")]);
        if status == "verified" {
            lines_include(&result, &["rva=0x00419210"]);
        }
    }
}

#[tokio::test]
async fn client_data_takes_precedence() {
    tool_error("get", json!({"kind":"spell", "id":90000}), "spell 90000 not found").await;
    tool_error("search", json!({"text":"Server Only"}), "no matches").await;
    let item = success("get", json!({"kind":"item", "id":50000, "fields":["all"]})).await;
    lines_include(&item, &["class=ARMOR", "display_id=1234"]);
    assert!(!item.lines().any(|line| line.starts_with("name=")), "{item}");
    lines_include(
        &success("get", json!({"kind":"item", "id":19019, "fields":["class"]})).await,
        &["class=WEAPON"],
    );
}

#[tokio::test]
async fn search_ranks_exact_and_prefix_matches_first() {
    let results = success("search", json!({"text":"fireball"})).await;
    let lines: Vec<_> = results.lines().collect();
    assert_eq!(
        &lines[..2],
        &["spell 133 Fireball (Rank 1)", "spell 143 Fireball (Rank 2)"]
    );
    assert!(
        lines
            .iter()
            .skip(2)
            .any(|line| line.starts_with("spell 56368 Glyph of Fireball")),
        "{results}"
    );
    assert!(
        lines
            .iter()
            .skip(2)
            .any(|line| line.starts_with("item 42739 Glyph of Fireball")),
        "{results}"
    );
    let fire = success("search", json!({"text":"fire", "kind":"spell"})).await;
    assert!(
        fire.find("Fire Blast").unwrap() < fire.find("Glyph of Fireball").unwrap(),
        "{fire}"
    );
    let health = success("search", json!({"text":"health", "kind":"lua_function"})).await;
    assert!(
        health
            .lines()
            .any(|line| line.starts_with("lua_function UnitHealth (health = UnitHealth")),
        "{health}"
    );
}

#[expect(
    clippy::too_many_lines,
    reason = "one table-driven test covers all relation fixtures"
)]
#[tokio::test]
async fn all_relations_return_linked_fixture_records() {
    let cases: &[(&str, i64, &str, &[&str])] = &[
        (
            "item",
            19019,
            "dropped_by",
            &["creature 12056 Baron Geddon chance=4.5 count=1"],
        ),
        ("item", 19020, "dropped_by", &["reference 10000 chance=10 count=1"]),
        (
            "item",
            4002,
            "dropped_by",
            &[
                "gameobject 2843 Battered Chest chance=50 count=1-3",
                "item 4008 Bag of Goodies chance=100 count=1",
            ],
        ),
        (
            "item",
            19020,
            "sold_by",
            &[
                "creature 5000 Brew Vendor",
                "cost=honor:100,4002x2",
                "max_count=1",
                "restock_time=3600",
            ],
        ),
        (
            "item",
            4002,
            "reward_of",
            &["quest 1001 The Fiery Path role=reward count=2"],
        ),
        ("item", 4004, "reward_of", &["role=choice"]),
        (
            "item",
            4004,
            "set",
            &[
                "itemset 1 Fiery Regalia",
                "item 4004 Fiery Cowl",
                "item 4005 Fiery Robe",
                "spell 4006 Fiery Regalia Bonus pieces=2",
            ],
        ),
        ("item", 4002, "created_by", &["spell 4001 Fiery Brew count=1"]),
        (
            "item",
            19019,
            "disenchant",
            &["item 20725 Nexus Crystal chance=100 count=1"],
        ),
        (
            "creature",
            12056,
            "loot",
            &[
                "item 19019 Thunderfury, Blessed Blade of the Windseeker chance=4.5 count=1",
                "reference 10000 chance=100 count=1",
            ],
        ),
        (
            "gameobject",
            2843,
            "loot",
            &["item 4002 Fiery Brew Potion chance=50 count=1-3"],
        ),
        (
            "creature",
            5000,
            "sells",
            &[
                "item 4002 Fiery Brew Potion",
                "item 19020 Tyrael's Hilt",
                "cost=honor:100,4002x2",
            ],
        ),
        (
            "creature",
            5001,
            "trains",
            &["spell 133 Fireball rank=Rank 1 cost=100 req_level=1"],
        ),
        (
            "creature",
            5002,
            "starts_quests",
            &["quest 1001 The Fiery Path level=10"],
        ),
        ("creature", 5002, "ends_quests", &["quest 1001 The Fiery Path level=10"]),
        (
            "creature",
            12056,
            "spawns",
            &["spawn 1 Molten Core map=409 zone=2717 area=2717 x=759.5 y=-1040.25 z=-133.5 o=2 spawn_time=604800"],
        ),
        ("gameobject", 2843, "spawns", &["spawn 1 Elwynn Forest"]),
        ("creature", 12056, "spells", &["spell 133 Fireball rank=Rank 1"]),
        (
            "quest",
            1001,
            "givers",
            &["creature 5002 Marshal Dughan", "item 4007 Mysterious Letter"],
        ),
        ("quest", 1002, "enders", &["gameobject 2843 Battered Chest"]),
        (
            "quest",
            1002,
            "chain",
            &[
                "quest 1001 The Fiery Path link=previous",
                "quest 1003 Another Fiery Path link=exclusive",
            ],
        ),
        (
            "quest",
            1001,
            "chain",
            &["quest 1002 The Fiery Path Continues link=next", "link=follow_up"],
        ),
        (
            "quest",
            1001,
            "rewards",
            &[
                "item 4002 Fiery Brew Potion count=2",
                "item 4004 Fiery Cowl count=1 choice=1",
                "spell 4001 Fiery Brew",
                "faction 72 Stormwind value_id=5",
            ],
        ),
        (
            "quest",
            1001,
            "requirements",
            &[
                "creature 12056 Baron Geddon count=1",
                "gameobject 2843 Battered Chest count=1",
                "item 4002 Fiery Brew Potion count=3",
                "item 4007 Mysterious Letter count=1 provided=1",
            ],
        ),
        (
            "spell",
            143,
            "ranks",
            &["spell 133 Fireball rank=1", "spell 143 Fireball rank=2"],
        ),
        (
            "spell",
            4001,
            "taught_by",
            &[
                "creature 5001 Fire Trainer cost=500 req_level=10",
                "item 4000 Recipe: Fiery Brew",
                "spell 4003 Teach Fiery Brew",
                "quest 1001 The Fiery Path",
            ],
        ),
        (
            "spell",
            483,
            "used_by",
            &["item 4000 Recipe: Fiery Brew trigger=ON_USE"],
        ),
        ("spell", 133, "used_by", &["creature 12056 Baron Geddon"]),
        (
            "spell",
            11366,
            "talent",
            &["talent 36 Pyroblast rank=1 tab=Fire class=MAGE column_index=1"],
        ),
        (
            "spell",
            56368,
            "glyph",
            &["glyph 1 Glyph of Fireball type=MAJOR", "item 42739 Glyph of Fireball"],
        ),
        (
            "spell",
            2136,
            "triggered",
            &["spell 133 Fireball effect_index=1 effect=TRIGGER_SPELL"],
        ),
        ("area", 12, "children", &["area 87 Goldshire"]),
        (
            "area",
            87,
            "parent",
            &["area 12 Elwynn Forest", "map 0 Eastern Kingdoms"],
        ),
        (
            "faction",
            72,
            "members",
            &[
                "creature 5000 Brew Vendor",
                "creature 5001 Fire Trainer",
                "creature 5002 Marshal Dughan",
            ],
        ),
        (
            "achievement",
            6,
            "criteria",
            &["criteria 1 Reach level 10 type=REACH_LEVEL quantity=10"],
        ),
    ];
    for &(kind, id, relation, expected) in cases {
        let result = success("related", json!({"kind":kind,"id":id,"relation":relation})).await;
        for part in expected {
            assert!(
                result.contains(part),
                "{kind} {id} {relation}: missing {part:?} in {result:?}"
            );
        }
        if relation == "set" || relation == "ranks" {
            let mut previous = 0;
            for part in expected {
                let index = result.find(part).unwrap();
                assert!(index >= previous, "{result}");
                previous = index;
            }
        }
    }
}

#[tokio::test]
async fn sql_is_read_only_bounded_and_reports_errors() {
    assert_eq!(
        success(
            "sql",
            json!({"query":"SELECT id, name FROM spell ORDER BY id", "limit":2})
        )
        .await,
        "id\tname\n133\tFireball\n143\tFireball\n-- truncated: first 2 rows shown; raise limit (max 1000) or narrow the query"
    );
    for query in ["DELETE FROM spell", "SELECT 1; SELECT 2", "SELEC 1"] {
        invalid("sql", json!({"query":query})).await;
    }
    invalid("sql", json!({"query":"SELECT 1", "limit":1001})).await;
    tool_error("sql", json!({"query":"SELECT abs(-9223372036854775808)"}), "SQL error:").await;
    tool_error(
        "sql",
        json!({"query":"WITH RECURSIVE c(x) AS (SELECT 1 UNION ALL SELECT x + 1 FROM c) SELECT count(*) FROM c"}),
        "2 seconds",
    )
    .await;
    assert!(
        success(
            "sql",
            json!({"query":"SELECT sql FROM sqlite_schema WHERE name = 'spell'"})
        )
        .await
        .contains("-- ")
    );
}

#[tokio::test]
async fn invalid_requests_and_missing_data_are_distinct() {
    for (tool, args) in [
        ("get", json!({"kind":"pet","id":1})),
        ("related", json!({"kind":"item","id":19019,"relation":"eaten_by"})),
        ("related", json!({"kind":"item","id":19019,"relation":"trains"})),
        ("related", json!({"kind":"offset","id":"X","relation":"ranks"})),
        ("get", json!({"kind":"item"})),
        ("get", json!({"kind":"item","id":"abc"})),
        ("get", json!({"kind":"item","id":19019,"fields":["nope"]})),
    ] {
        invalid(tool, args).await;
    }
    tool_error("get", json!({"kind":"item","id":999_999}), "item 999999 not found").await;
    tool_error(
        "related",
        json!({"kind":"creature","id":5002,"relation":"loot"}),
        "no loot for creature 5002",
    )
    .await;
    tool_error("search", json!({"text":"zzzz"}), "no matches").await;
}

#[tokio::test]
async fn meta_and_default_values_are_rendered_correctly() {
    let meta = success("get", json!({"kind":"meta"})).await;
    lines_include(
        &meta,
        &[
            "tdb_release=TDB 335.26091",
            "client_build=12340",
            "codex_commit=1111111111111111111111111111111111111111",
            "offsets_copied=2026-10-18",
            "builder_version=0.1.0",
        ],
    );
    let hash = meta
        .lines()
        .find_map(|line| line.strip_prefix("tdb_sha256="))
        .expect("dump hash");
    assert_eq!(hash.len(), 64);
    assert!(
        hash.bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    );
    let item = success("get", json!({"kind":"item","id":19020,"fields":["all"]})).await;
    for field in ["required_level=", "stackable=", "armor=", "description=", "buy_count="] {
        assert!(!item.lines().any(|line| line.starts_with(field)), "{item}");
    }
    lines_include(
        &success("get", json!({"kind":"cvar","id":"autoLootDefault"})).await,
        &["default_value=0"],
    );
}

#[tokio::test]
async fn codex_records_are_available_by_name() {
    let function = success("get", json!({"kind":"lua_function","id":"UnitHealth"})).await;
    lines_include(&function, &["signature=health = UnitHealth(\"unit\")"]);
    assert!(function.contains("parameters=unit (string)"), "{function}");
    assert!(function.contains("returns=health (number)"), "{function}");
    let widget = success("get", json!({"kind":"widget","id":"Frame"})).await;
    lines_include(&widget, &["inherits=Region", "methods=Frame:Show()"]);
    let event = success("get", json!({"kind":"combatlog_event","id":"SPELL_DAMAGE"})).await;
    lines_include(&event, &["type=suffix", "prefixes=SPELL"]);
    lines_include(
        &success("get", json!({"kind":"combatlog_event","id":"SPELL"})).await,
        &["type=prefix"],
    );
    for (kind, id) in [
        ("lua_event", "PLAYER_LEVEL_UP"),
        ("lua_type", "unitId"),
        ("secure_template", "SecureActionButtonTemplate"),
    ] {
        let result = success("get", json!({"kind":kind,"id":id})).await;
        assert!(result.contains(id), "{result}");
    }
}

#[tokio::test]
async fn spell_effect_misc_values_preserve_zero() {
    let result = success(
        "sql",
        json!({"query":"SELECT misc_value, misc_value_b FROM spell_effect WHERE spell_id=133 AND idx=1"}),
    )
    .await;
    assert_eq!(result, "misc_value\tmisc_value_b\n0\t0");
}

#[tokio::test]
async fn item_spell_sentinels_are_not_imported() {
    let item = success("get", json!({"kind":"item","id":4000,"fields":["spells"]})).await;
    assert!(item.contains("spells="), "{item}");
    assert!(!item.contains("-1"), "{item}");
    let count = success(
        "sql",
        json!({"query":"SELECT count(*) AS negative_ids FROM item_spell WHERE spell_id < 0"}),
    )
    .await;
    assert_eq!(count, "negative_ids\n0");
}

#[tokio::test]
async fn nested_vendor_references_expose_stock_in_both_directions() {
    let sells = success("related", json!({"kind":"creature","id":5002,"relation":"sells"})).await;
    lines_include(
        &sells,
        &[
            "item 4002 Fiery Brew Potion",
            "item 19020 Tyrael's Hilt cost=honor:100,4002x2 max_count=1 restock_time=3600",
        ],
    );
    let sold_by = success("related", json!({"kind":"item","id":4002,"relation":"sold_by"})).await;
    lines_include(
        &sold_by,
        &[
            "creature 5000 Brew Vendor",
            "creature 5001 Fire Trainer",
            "creature 5002 Marshal Dughan",
        ],
    );
}
