use super::Config;

#[test]
fn test_config() {
    let config: Config = serde_json::from_str(
    r#"
    {
        "excludes": [
            "a",
            "b",
            "c"
        ]
    }
    "#,
    )
    .unwrap();
    println!("{:?}", config);
}
