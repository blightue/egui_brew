// use serde;
// use serde::{Deserialize, Serialize};

// pub type Formulae = Vec<Formula>;

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Formula {
//     pub name: String,
//     #[serde(rename = "full_name")]
//     pub full_name: String,
//     pub tap: String,
//     pub oldname: String,
//     pub oldnames: Vec<String>,
//     pub aliases: Vec<String>,
//     #[serde(rename = "versioned_formulae")]
//     pub versioned_formulae: Vec<Value>,
//     pub desc: String,
//     pub license: String,
//     pub homepage: String,
//     pub versions: Versions,
//     pub urls: Urls,
//     pub revision: i64,
//     #[serde(rename = "version_scheme")]
//     pub version_scheme: i64,
//     pub bottle: Bottle,
//     #[serde(rename = "keg_only")]
//     pub keg_only: bool,
//     #[serde(rename = "keg_only_reason")]
//     pub keg_only_reason: Value,
//     pub options: Vec<Value>,
//     #[serde(rename = "build_dependencies")]
//     pub build_dependencies: Vec<String>,
//     pub dependencies: Vec<String>,
//     #[serde(rename = "test_dependencies")]
//     pub test_dependencies: Vec<String>,
//     #[serde(rename = "recommended_dependencies")]
//     pub recommended_dependencies: Vec<Value>,
//     #[serde(rename = "optional_dependencies")]
//     pub optional_dependencies: Vec<Value>,
//     #[serde(rename = "uses_from_macos")]
//     pub uses_from_macos: (UsesFromMacos, String),
//     #[serde(rename = "uses_from_macos_bounds")]
//     pub uses_from_macos_bounds: Vec<UsesFromMacosBound>,
//     pub requirements: Vec<Value>,
//     #[serde(rename = "conflicts_with")]
//     pub conflicts_with: Vec<Value>,
//     #[serde(rename = "conflicts_with_reasons")]
//     pub conflicts_with_reasons: Vec<Value>,
//     #[serde(rename = "link_overwrite")]
//     pub link_overwrite: Vec<Value>,
//     pub caveats: Value,
//     pub installed: Vec<Installed>,
//     #[serde(rename = "linked_keg")]
//     pub linked_keg: String,
//     pub pinned: bool,
//     pub outdated: bool,
//     pub deprecated: bool,
//     #[serde(rename = "deprecation_date")]
//     pub deprecation_date: Value,
//     #[serde(rename = "deprecation_reason")]
//     pub deprecation_reason: Value,
//     pub disabled: bool,
//     #[serde(rename = "disable_date")]
//     pub disable_date: Value,
//     #[serde(rename = "disable_reason")]
//     pub disable_reason: Value,
//     #[serde(rename = "post_install_defined")]
//     pub post_install_defined: bool,
//     pub service: Value,
//     #[serde(rename = "tap_git_head")]
//     pub tap_git_head: String,
//     #[serde(rename = "ruby_source_path")]
//     pub ruby_source_path: String,
//     #[serde(rename = "ruby_source_checksum")]
//     pub ruby_source_checksum: RubySourceChecksum,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Versions {
//     pub stable: String,
//     pub head: Value,
//     pub bottle: bool,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Urls {
//     pub stable: Stable,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Stable {
//     pub url: String,
//     pub tag: Value,
//     pub revision: Value,
//     pub using: Value,
//     pub checksum: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Bottle {
//     pub stable: Stable2,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Stable2 {
//     pub rebuild: i64,
//     #[serde(rename = "root_url")]
//     pub root_url: String,
//     pub files: Files,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Files {
//     #[serde(rename = "arm64_sonoma")]
//     pub arm64_sonoma: Arm64Sonoma,
//     #[serde(rename = "arm64_ventura")]
//     pub arm64_ventura: Arm64Ventura,
//     #[serde(rename = "arm64_monterey")]
//     pub arm64_monterey: Arm64Monterey,
//     pub sonoma: Sonoma,
//     pub ventura: Ventura,
//     pub monterey: Monterey,
//     #[serde(rename = "x86_64_linux")]
//     pub x86_64_linux: X8664Linux,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Arm64Sonoma {
//     pub cellar: String,
//     pub url: String,
//     pub sha256: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Arm64Ventura {
//     pub cellar: String,
//     pub url: String,
//     pub sha256: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Arm64Monterey {
//     pub cellar: String,
//     pub url: String,
//     pub sha256: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Sonoma {
//     pub cellar: String,
//     pub url: String,
//     pub sha256: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Ventura {
//     pub cellar: String,
//     pub url: String,
//     pub sha256: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Monterey {
//     pub cellar: String,
//     pub url: String,
//     pub sha256: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct X8664Linux {
//     pub cellar: String,
//     pub url: String,
//     pub sha256: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct UsesFromMacos {
//     pub libxslt: String,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct UsesFromMacosBound {}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Installed {
//     pub version: String,
//     #[serde(rename = "used_options")]
//     pub used_options: Vec<Value>,
//     #[serde(rename = "built_as_bottle")]
//     pub built_as_bottle: bool,
//     #[serde(rename = "poured_from_bottle")]
//     pub poured_from_bottle: bool,
//     pub time: i64,
//     #[serde(rename = "runtime_dependencies")]
//     pub runtime_dependencies: Vec<RuntimeDependency>,
//     #[serde(rename = "installed_as_dependency")]
//     pub installed_as_dependency: bool,
//     #[serde(rename = "installed_on_request")]
//     pub installed_on_request: bool,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct RuntimeDependency {
//     #[serde(rename = "full_name")]
//     pub full_name: String,
//     pub version: String,
//     pub revision: i64,
//     #[serde(rename = "pkg_version")]
//     pub pkg_version: String,
//     #[serde(rename = "declared_directly")]
//     pub declared_directly: bool,
// }

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct RubySourceChecksum {
//     pub sha256: String,
// }
