use crate::WhitespaceSifter;

#[test]
fn test_sift() {
    let input: String = format!(
        "{}\n\n{}\n\n{}\n\r\n\n{}\r\n\n\r\n{}\r\n\r\n{}\r\n\r\n\r\n",
        "This\u{0020}\u{0020}is\u{0020}\u{0020}\u{0020}a\u{0020}\u{0020}sentence...",
        "❤️With\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}some\u{0020}\u{0020}duplicate...",
        "Whitespaces.",
        "This\u{0020}\u{0020}is\u{0020}\u{0020}\u{0020}a\u{0020}\u{0020}sentence...",
        "With\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}some\u{0020}\u{0020}duplicate...",
        "Whitespaces."
    );
    assert_eq!(
        &input.sift(),
        "This is a sentence...\n❤️With some duplicate...\nWhitespaces.\nThis is a sentence...\r\nWith some duplicate...\r\nWhitespaces."
    );
}

#[test]
fn test_sift_preserved() {
    let input: String = format!(
        "{}\n\n{}\n\n{}\n\n\n{}\r\n\n\r\n{}\r\n\r\n{}\r\n\r\n\r\n",
        "This. \n\nis. \n\na. \n\nsentence... \n\n",
        "✨With. \n\nsome. \n\nduplicate... \n\n",
        "Whitespaces. \n\n",
        "This. \r\n\r\nis. \r\n\r\na. \r\n\r\nsentence... \r\n\r\n",
        "With. \r\n\r\nsome. \r\n\r\nduplicate... \r\n\r\n",
        "Whitespaces. \r\n\r\n"
    );
    assert_eq!(
        &input.sift_preserve_newlines(),
        "This.\nis.\na.\nsentence...\n✨With.\nsome.\nduplicate...\nWhitespaces.\nThis.\r\nis.\r\na.\r\nsentence...\r\nWith.\r\nsome.\r\nduplicate...\r\nWhitespaces."
    );
}

#[test]
fn test_blank_string_sifting() {
    assert_eq!(&"".sift(), "");
    assert_eq!(&"".sift_preserve_newlines(), "");
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
