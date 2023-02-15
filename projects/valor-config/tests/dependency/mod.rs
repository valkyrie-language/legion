use std::collections::BTreeMap;
use valor_config::ValorDependency;

#[test]
fn test() {
    let toml = r#"
    name = "^1.0"
    "#;
    println!("{:#?}", toml::from_str::<BTreeMap<String, ValorDependency>>(toml));
}
