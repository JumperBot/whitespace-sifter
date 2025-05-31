use std::process::{Command, Output};

const VERSIONS: [&str; 7] = [
    "2.3.5", "2.3.4", "2.3.3", "2.3.2", "2.3.1", "2.2.0", "2.1.0",
];

#[test]
fn test_old_versions() {
    let cargo_new: Output = Command::new("cargo")
        .args(["new", "--lib", "compliance_tester_temp"])
        .output()
        .expect("Failed to test old versions");

    assert!(cargo_new.status.success());

    std::fs::write(
        "compliance_tester_temp/src/lib.rs",
        include_str!("tests.rs")
            .replace(
                "use crate::WhitespaceSifter;",
                "#[allow(unused_imports)] use whitespace_sifter::WhitespaceSifter;",
            )
            .as_bytes(),
    )
    .expect("Failed to inject test code");

    for ver in VERSIONS.iter().rev() {
        std::fs::write(
            "compliance_tester_temp/Cargo.toml",
            &format!(
                "
                [package]
                name = \"compliance_tester_temp\"
                version = \"0.1.0\"
                edition = \"2021\"

                [dependencies]
                whitespace-sifter = \"={ver}\"
                "
            ),
        )
        .expect("Failed to inject Cargo.toml base");

        let cargo_test: Output = Command::new("cargo")
            .arg("test")
            .current_dir(
                std::env::current_dir()
                    .expect("Failed to get current directory")
                    .join("compliance_tester_temp"),
            )
            .output()
            .expect("Failed to run cargo test");

        assert!(!String::from_utf8_lossy(&cargo_test.stdout).contains("FAIL"));
        assert!(!String::from_utf8_lossy(&cargo_test.stderr).contains("FAIL"));
    }

    std::fs::remove_dir_all(
        std::env::current_dir()
            .expect("Failed to get current directory")
            .join("compliance_tester_temp"),
    )
    .expect("Failed to remove compliance_tester_temp directory");
}
