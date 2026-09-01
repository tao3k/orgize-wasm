use std::process::Command;

fn main() {
    asp_rust_build_support::emit_provider_contract_digest();
    orgize_wasm_build_support::enforce_public_surface_file_shape(env!("CARGO_MANIFEST_DIR"));

    {
        let output = Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .output()
            .unwrap();

        let git_hash = String::from_utf8(output.stdout).unwrap();

        println!("cargo:rustc-env=CARGO_GIT_HASH={}", git_hash);
    }

    {
        let output = Command::new("date").args(["-R"]).output().unwrap();

        let git_hash = String::from_utf8(output.stdout).unwrap();

        println!("cargo:rustc-env=CARGO_BUILD_TIME={}", git_hash);
    }
}
