use std::process::{Command, Output};

#[test]
fn verify_msrv() {
    let verify_out: Output = Command::new("cargo")
        .args(["msrv", "verify"])
        .output()
        .expect("Failed to verify MSRV");

    assert!(verify_out.status.success());

    let cmp: &str = "OK";
    assert!(verify_out
        .stdout
        .windows(cmp.len())
        .any(|window| window == cmp.as_bytes()));

    let cmp: &str = "Is compatible";
    assert!(verify_out
        .stdout
        .windows(cmp.len())
        .any(|window| window == cmp.as_bytes()));
}
