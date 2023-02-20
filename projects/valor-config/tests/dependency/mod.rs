use valor_config::ValorConfig;

#[test]
fn test() {
    let toml = r#"
    [dependencies]
    "dp1" = "^1.0"
    "@a/dp2" = { path = "path/to/dep" }
    
    [dependencies.dp3]
    version = "^1.0.2-alpha"
    "#;
    println!("{:#?}", toml::from_str::<ValorConfig>(toml).unwrap());
}
