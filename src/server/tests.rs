use std::{sync::mpsc, time::Duration};

use rmcp::{
    ServiceExt,
    model::{CallToolRequestParams, CallToolResult},
    serde_json::json,
};

use super::WotlkServer;
use crate::provision::tests::{Fixture, serve_once};

type Client = rmcp::service::RunningService<rmcp::RoleClient, ()>;

async fn connect(server: WotlkServer) -> (Client, tokio::task::JoinHandle<()>) {
    let (server_io, client_io) = tokio::io::duplex(65536);
    let task = tokio::spawn(async move {
        server
            .serve(server_io)
            .await
            .expect("serve")
            .waiting()
            .await
            .expect("wait for server");
    });
    let client = ().serve(client_io).await.expect("initialize client");
    (client, task)
}

async fn call(client: &Client) -> CallToolResult {
    client
        .call_tool(
            CallToolRequestParams::new("sql").with_arguments(
                json!({"query":"SELECT value FROM meta WHERE name='client_build'"})
                    .as_object()
                    .expect("arguments")
                    .clone(),
            ),
        )
        .await
        .expect("call SQL tool")
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

#[tokio::test]
async fn handshake_and_tools_list_do_not_wait_for_download_and_tools_recover_without_restart() {
    let fixture = Fixture::new();
    let (release, waiting) = mpsc::channel();
    let (url, http) = serve_once(fixture.compressed.clone(), Some(waiting));
    let server = WotlkServer::with_provisioner(fixture.provisioner(url));
    let (client, task) = tokio::time::timeout(Duration::from_secs(2), connect(server))
        .await
        .expect("handshake before download");
    let tools = client.list_tools(None).await.expect("list tools while loading");
    assert_eq!(tools.tools.len(), 4);
    let loading = tokio::time::timeout(Duration::from_secs(2), async {
        loop {
            let result = call(&client).await;
            if text(&result).contains("40%") {
                break result;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .expect("compressed byte progress reaches 40%");
    assert_eq!(loading.is_error, Some(true));
    let message = text(&loading);
    assert!(
        message.contains("downloading") && message.contains("40%") && message.contains("retry"),
        "{message}"
    );
    release.send(()).expect("release download");
    let ready = tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let result = call(&client).await;
            if result.is_error != Some(true) {
                break result;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .expect("database becomes ready");
    assert_eq!(text(&ready), "value\n12340");
    client.cancel().await.expect("cancel client");
    task.await.expect("join server");
    http.join().expect("join HTTP server");
}

#[tokio::test]
async fn unreachable_host_becomes_tool_error_with_cause_url_and_manual_retry() {
    let fixture = Fixture::new();
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("reserve unreachable address");
    let url = format!("http://{}/database.zst", listener.local_addr().expect("address"));
    drop(listener);
    let server = WotlkServer::with_provisioner(fixture.provisioner(url.clone()));
    let (client, task) = connect(server).await;
    let failure = tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let result = call(&client).await;
            if !text(&result).contains("downloading") {
                break result;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .expect("download fails");
    let message = text(&failure);
    assert_eq!(failure.is_error, Some(true));
    assert!(
        message.contains("connect")
            && message.contains(&url)
            && message.contains("npx wotlk-database-mcp fetch")
            && message.contains("WOTLK_DB"),
        "{message}"
    );
    client.cancel().await.expect("cancel client");
    task.await.expect("join server");
}

#[tokio::test]
async fn no_checksum_build_still_serves_handshake_and_reports_actionable_tool_error() {
    let fixture = Fixture::new();
    let mut provisioner = fixture.provisioner("http://127.0.0.1:1/database.zst".to_owned());
    provisioner.sha256 = None;
    let (client, task) = connect(WotlkServer::with_provisioner(provisioner)).await;
    assert_eq!(client.list_tools(None).await.expect("list tools").tools.len(), 4);
    let failure = tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            let result = call(&client).await;
            if !text(&result).contains("downloading") {
                break result;
            }
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .expect("checksum failure becomes visible");
    let message = text(&failure);
    assert!(
        message.contains("no embedded database checksum") && message.contains("--db") && message.contains("WOTLK_DB"),
        "{message}"
    );
    client.cancel().await.expect("cancel client");
    task.await.expect("join server");
}
