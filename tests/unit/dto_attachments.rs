use std::{fs, path::Path};

use orgize_wasm::Org;
use serde_json::{Value, json};

#[test]
fn wasm_attachment_inventory_contract_exposes_display_and_sync_plan() {
    let temp = unique_temp_dir("orgize-wasm-attachment-inventory");
    fs::create_dir_all(temp.join(".attach/aa/bbccdd-0000-4000-8000-000000000000"))
        .expect("create id attachment directory");
    fs::write(
        temp.join(".attach/aa/bbccdd-0000-4000-8000-000000000000/wallpaper.jpg"),
        b"wallpaper",
    )
    .expect("write wallpaper");
    fs::write(
        temp.join(".attach/aa/bbccdd-0000-4000-8000-000000000000/orphan.jpg"),
        b"orphan",
    )
    .expect("write orphan");

    let org = Org::parse(
        r#"* Wallpaper :ATTACH:
:PROPERTIES:
:ID: aabbccdd-0000-4000-8000-000000000000
:END:
[[attachment:wallpaper.jpg]]
[[attachment:missing.jpg]]
"#,
    );
    let request = json!({
        "baseDir": path_str(&temp),
        "attachIdDir": ".attach",
        "scanOrphans": true
    })
    .to_string();
    let payload: Value = serde_json::from_str(
        &org.attachment_inventory_json(Some(request))
            .expect("attachment inventory JSON should render"),
    )
    .expect("attachment inventory JSON should parse");

    assert_eq!(payload["schemaVersion"], 1);
    assert_eq!(payload["display"][0]["sectionTitle"], "Wallpaper");
    assert_eq!(payload["display"][0]["mediaKind"], "image");
    assert_eq!(
        payload["display"][0]["attachmentId"],
        "aabbccdd-0000-4000-8000-000000000000"
    );
    assert_eq!(
        payload["display"][0]["directoryPath"],
        ".attach/aa/bbccdd-0000-4000-8000-000000000000"
    );
    assert!(
        payload["syncPlan"]["actions"]
            .as_array()
            .expect("sync actions")
            .iter()
            .any(|action| action["kind"] == "missingLinkedFile" && action["path"] == "missing.jpg")
    );
    assert!(
        payload["syncPlan"]["actions"]
            .as_array()
            .expect("sync actions")
            .iter()
            .any(|action| action["kind"] == "orphanFile"
                && action["path"]
                    .as_str()
                    .expect("orphan path")
                    .ends_with("/orphan.jpg"))
    );

    let _ = fs::remove_dir_all(temp);
}

fn unique_temp_dir(label: &str) -> std::path::PathBuf {
    let pid = std::process::id();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("{label}-{pid}-{nanos}"))
}

fn path_str(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}
