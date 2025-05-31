fn main() {
    steamkit_vdf::from_str(
        r#"
        "key" "value"
        "key2" {
            "key3" "value3"
            "key4" {
                "key5" "value5"
            }
        }"#,
        &steamkit_vdf::Options::default(),
    )
    .unwrap();
}
