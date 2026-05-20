//! Build-time policy checks for the orgize WebAssembly package.

use std::{fs, path::Path};

/// Enforces local public-surface file budgets that are outside the Rust-only
/// parser scope of `rust-lang-project-harness`.
pub fn enforce_public_surface_file_shape(package_root: &str) {
    const MAX_DTO_BARREL_LINES: usize = 260;
    const MAX_DTO_MODULE_LINES: usize = 650;
    const MAX_WASM_BINDING_LINES: usize = 900;
    const MAX_WORKER_SURFACE_LINES: usize = 350;

    let root = Path::new(package_root);
    let fixed_limits = [
        ("dto.d.ts", MAX_DTO_BARREL_LINES),
        ("worker.d.ts", MAX_DTO_BARREL_LINES),
        ("worker.js", MAX_WORKER_SURFACE_LINES),
        ("src/dto_model.rs", MAX_WASM_BINDING_LINES),
        ("src/bindings.rs", MAX_WASM_BINDING_LINES),
    ];
    for (relative, max_lines) in fixed_limits {
        assert_file_line_budget(root, relative, max_lines);
    }

    let dto_dir = root.join("dto");
    println!("cargo:rerun-if-changed={}", dto_dir.display());
    let entries = fs::read_dir(&dto_dir).unwrap_or_else(|error| {
        panic!(
            "failed to read public DTO declaration directory {}: {error}",
            dto_dir.display()
        )
    });
    for entry in entries {
        let path = entry
            .unwrap_or_else(|error| panic!("failed to read DTO declaration entry: {error}"))
            .path();
        if path.extension().and_then(|value| value.to_str()) != Some("ts") {
            continue;
        }
        let relative = path
            .strip_prefix(root)
            .expect("DTO path should be under package root")
            .to_string_lossy()
            .into_owned();
        assert_file_line_budget(root, relative.as_str(), MAX_DTO_MODULE_LINES);
    }
}

fn assert_file_line_budget(root: &Path, relative: &str, max_lines: usize) {
    let path = root.join(relative);
    println!("cargo:rerun-if-changed={}", path.display());
    let source = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
    let line_count = source.lines().count();
    assert!(
        line_count <= max_lines,
        "{} has {line_count} lines, above the {max_lines}-line public surface budget; split the DTO/binding surface instead of growing this file",
        path.display()
    );
}
