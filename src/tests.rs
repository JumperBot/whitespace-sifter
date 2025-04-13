use crate::WhitespaceSifter;

use std::process::{Command, Output};

#[test]
fn test_unicode_compatibility() {
    let input: String = "  a1❤️🌐🚀1a  ".to_owned();
    let out: String = "a1❤️🌐🚀1a".to_owned();
    assert_eq!(input.sift(), out);
    assert_eq!(input.sift_preserve_newlines(), out);
}

#[test]
fn test_sift() {
    let input: String = "a\r\n\n\t b".to_owned();
    let out: String = "a\r\nb".to_owned();
    assert_eq!(input.sift(), out);
}

#[test]
fn test_sift_leading_whitespaces() {
    let input: String = "a\r\n\n\t b\r\n\r\n\r\n".to_owned();
    let out: String = "a\r\nb".to_owned();
    assert_eq!(input.sift(), out);
    let input: String = "a\r\n\n\t b\t ".to_owned();
    assert_eq!(input.sift(), out);
    let input: String = "a\r\n\n\t b\t \n".to_owned();
    assert_eq!(input.sift(), out);
}

#[test]
fn test_sift_preserve_newlines() {
    let input: String = "a \r\n\n\t b".to_owned();
    let out: String = "a\r\nb".to_owned();
    assert_eq!(input.sift_preserve_newlines(), out);
}

#[test]
fn test_sift_preserve_newlines_leading_whitespaces() {
    let input: String = "a \r\n\n\t b\r\n\r\n\r\n".to_owned();
    let out: String = "a\r\nb".to_owned();
    assert_eq!(input.sift_preserve_newlines(), out);
    let input: String = "a \r\n\n\t b\t ".to_owned();
    assert_eq!(input.sift_preserve_newlines(), out);
    let input: String = "a \r\n\n\t b\t \n".to_owned();
    assert_eq!(input.sift_preserve_newlines(), out);
}

#[test]
fn test_all_blank_output() {
    assert_eq!(&"".sift(), "");
    assert_eq!(&"\n\r\n".sift(), "");
    assert_eq!(&"\t ".sift(), "");
    assert_eq!(&"".sift_preserve_newlines(), "");
    assert_eq!(&"\n\r\n".sift_preserve_newlines(), "");
    assert_eq!(&"\t ".sift_preserve_newlines(), "");
}

#[test]
fn test_docs() {
    assert_eq!(
        &"1.. \n2..  \n\r\n\n3..   \n\n\n4..    \n\n\r\n\n\n5..     \n\n\n\n\n".sift(),
        "1.. 2.. 3.. 4.. 5.."
    );
    assert_eq!(
        &"1.. \n2..  \n\r\n3..   \n\n\n4..    \r\n\n\r\n\n5..     \n\n\n\n\n"
            .sift_preserve_newlines(),
        "1..\n2..\n3..\n4..\r\n5.."
    );
}

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
