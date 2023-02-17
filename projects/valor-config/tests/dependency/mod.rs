use valor_config::ValorConfig;

#[test]
fn test() {
    let toml = r#"
    [dependencies]
    name = "^1.0"
    "#;
    println!("{:#?}", toml::from_str::<ValorConfig>(toml).unwrap());
}
