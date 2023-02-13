use crate::ValorDependencies;

pub struct ValorPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub authors: Vec<String>,
    pub dependencies: Vec<ValorDependencies>,
    pub dev_dependencies: Vec<ValorDependencies>,
    pub build_dependencies: Vec<ValorDependencies>,
    pub scripts: Vec<String>,
    pub files: Vec<String>,
    pub main: String,
    pub bin: Vec<String>,
    pub keywords: Vec<String>,
    pub license: String,
    pub repository: String,
    pub homepage: String,
    pub bugs: String,
}

