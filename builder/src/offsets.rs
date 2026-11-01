use std::{fs, path::Path};

use rusqlite::{Connection, params};

use crate::BuildError;

pub(crate) fn import(conn: &Connection, dir: &Path) -> Result<String, BuildError> {
    let version_path = dir.join("VERSION");
    let version = fs::read_to_string(&version_path).map_err(|source| BuildError::Io {
        path: version_path.clone(),
        source,
    })?;
    let version = version.trim();
    if version.is_empty() {
        return Err(BuildError::Offsets {
            path: version_path,
            line: 0,
            message: "empty VERSION".into(),
        });
    }
    let mut count = 0;
    let mut insert = conn.prepare(
        "INSERT INTO offsets (name, domain, platform, va, rva, status, comment) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
    )?;
    for platform in sorted_entries(dir)? {
        let path = platform.path();
        if !platform
            .file_type()
            .map_err(|source| BuildError::Io {
                path: path.clone(),
                source,
            })?
            .is_dir()
        {
            continue;
        }
        let platform_name = platform.file_name().to_string_lossy().into_owned();
        if platform_name != "windows-x86" {
            return Err(BuildError::Offsets {
                path,
                line: 0,
                message: format!("unknown offsets platform '{platform_name}'"),
            });
        }
        for file in sorted_entries(&path)? {
            let path = file.path();
            if path.extension().is_none_or(|ext| ext != "md") {
                continue;
            }
            count += import_file(&mut insert, &path, &platform_name)?;
        }
    }
    if count == 0 {
        return Err(BuildError::Offsets {
            path: dir.to_path_buf(),
            line: 0,
            message: "no offset entries".into(),
        });
    }
    Ok(version.to_owned())
}

fn sorted_entries(dir: &Path) -> Result<Vec<fs::DirEntry>, BuildError> {
    let io_error = |source| BuildError::Io {
        path: dir.to_path_buf(),
        source,
    };
    let mut entries = fs::read_dir(dir)
        .map_err(io_error)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(io_error)?;
    entries.sort_by_key(fs::DirEntry::file_name);
    Ok(entries)
}

fn import_file(insert: &mut rusqlite::Statement<'_>, path: &Path, platform_name: &str) -> Result<usize, BuildError> {
    let mut count = 0;
    let contents = fs::read_to_string(path).map_err(|source| BuildError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let mut in_cpp = false;
    let mut namespaces = Vec::new();
    for (index, raw) in contents.lines().enumerate() {
        let line = raw.trim();
        if !in_cpp {
            if line == "```cpp" {
                in_cpp = true;
                namespaces.clear();
            }
            continue;
        }
        if line == "```" {
            in_cpp = false;
            continue;
        }
        if line.is_empty() {
            continue;
        }
        if let Some(namespace) = line.strip_prefix("namespace ").and_then(|s| s.strip_suffix(" {"))
            && valid_identifier(namespace)
        {
            namespaces.push(namespace);
            continue;
        }
        if line.starts_with('}') && namespaces.pop().is_some() {
            continue;
        }
        let parsed = line
            .strip_prefix("constexpr uint32_t ")
            .and_then(|s| s.split_once(" = "))
            .and_then(|(name, rest)| rest.split_once("; // ").map(|(addr, comment)| (name, addr, comment)))
            .and_then(|(name, addr, comment)| {
                if !valid_identifier(name)
                    || addr.len() != 10
                    || !addr.starts_with("0x")
                    || namespaces.len() < 2
                    || namespaces[0] != "WoWOffsets"
                {
                    return None;
                }
                u32::from_str_radix(&addr[2..], 16)
                    .ok()
                    .and_then(|va| va.checked_sub(0x0040_0000).map(|rva| (name, va, rva, comment)))
            });
        let Some((name, va, rva, comment)) = parsed else {
            return Err(BuildError::Offsets {
                path: path.to_path_buf(),
                line: index + 1,
                message: "unrecognized line".into(),
            });
        };
        let status = if comment.contains("Not present") {
            "not_present"
        } else if comment.contains("label unverified") {
            "label_unverified"
        } else if comment.contains("Uncertain") {
            "uncertain"
        } else if comment.contains("UNVERIFIED") {
            "unverified"
        } else {
            "verified"
        };
        let qualified = format!("{}::{name}", namespaces.join("::"));
        insert.execute(params![
            qualified,
            namespaces[1],
            platform_name,
            format!("0x{va:08X}"),
            format!("0x{rva:08X}"),
            status,
            comment
        ])?;
        count += 1;
    }
    Ok(count)
}

fn valid_identifier(s: &str) -> bool {
    let mut chars = s.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}
