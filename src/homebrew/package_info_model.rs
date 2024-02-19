use async_trait::async_trait;
use serde;
use serde::{Deserialize, Serialize};

use super::async_loader::Load;
use super::package_model::PackageBrief;

pub trait PackageInfo {
    fn get_name(&self) -> &str;
    fn get_version(&self) -> &str;
    fn get_desc(&self) -> &str;
    fn get_homepage(&self) -> &str;
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormualVersion {
    pub stable: String,
    pub head: Option<String>,
    pub bottle: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormulaInfo {
    pub name: String,
    pub full_name: String,
    pub oldnames: Vec<String>,
    pub aliases: Vec<String>,
    pub desc: String,
    pub license: String,
    pub homepage: String,
    pub versions: FormualVersion,
}

impl PackageInfo for FormulaInfo {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_version(&self) -> &str {
        &self.versions.stable
    }
    fn get_desc(&self) -> &str {
        &self.desc
    }
    fn get_homepage(&self) -> &str {
        &self.homepage
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaskInfo {
    pub name: Vec<String>,
    pub full_token: String,
    pub old_tokens: Vec<String>,
    pub desc: Option<String>,
    pub homepage: String,
    pub version: String,
}

impl PackageInfo for CaskInfo {
    fn get_name(&self) -> &str {
        &self.name[0]
    }
    fn get_version(&self) -> &str {
        &self.version
    }
    fn get_desc(&self) -> &str {
        if let Some(desc) = &self.desc {
            desc
        } else {
            "No description"
        }
    }
    fn get_homepage(&self) -> &str {
        &self.homepage
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageInfoQuery {
    pub formulae: Vec<FormulaInfo>,
    pub casks: Vec<CaskInfo>,
}

pub type PackageInfoLoader = super::async_loader::AsyncLoader<PackageInfoHolder>;

pub struct PackageInfoHolder {
    package_brief: PackageBrief,
    pub package_info: Option<PackageInfoQuery>,
}

impl PackageInfoHolder {
    pub fn new(package_brief: PackageBrief) -> Self {
        Self {
            package_brief: package_brief,
            package_info: None,
        }
    }
    fn from_json(&mut self, json: &str) {
        let package_info: PackageInfoQuery = serde_json::from_str(json).unwrap();
        self.package_info = Some(package_info);
    }
}

#[async_trait]
impl Load for PackageInfoHolder {
    async fn load(&mut self) {
        let package_info = super::brew_cli::BrewCli::show_info(
            &self.package_brief.name.clone(),
            self.package_brief.package_type.clone(),
        )
        .await
        .unwrap();
        // println!("{}", package_info.result);
        self.from_json(&package_info.result);
    }
}

#[cfg(test)]
mod test {
    use crate::homebrew::package_info_model::PackageInfoQuery;

    #[test]
    fn test_package_info() {
        let json = r#"
        {
          "formulae": [
        
          ],
          "casks": [
            {
              "token": "unity-hub",
              "full_token": "unity-hub",
              "old_tokens": [
        
              ],
              "tap": "homebrew/cask",
              "name": [
                "Unity Hub"
              ],
              "desc": "Management tool for Unity",
              "homepage": "https://unity3d.com/get-unity/download",
              "url": "https://public-cdn.cloud.unity3d.com/hub/prod/UnityHubSetup.dmg",
              "url_specs": {
              },
              "appcast": null,
              "version": "3.7.0",
              "installed": "3.4.1",
              "installed_time": 1676358791,
              "outdated": false,
              "sha256": "no_check",
              "artifacts": [
                {
                  "uninstall": [
                    {
                      "quit": "com.unity3d.unityhub"
                    }
                  ]
                },
                {
                  "app": [
                    "Unity Hub.app"
                  ]
                },
                {
                  "zap": [
                    {
                      "trash": [
                        "~/Library/Preferences/com.unity3d.unityhub.helper.plist",
                        "~/Library/Preferences/com.unity3d.unityhub.plist"
                      ],
                      "rmdir": "/Applications/Unity/Hub"
                    }
                  ]
                }
              ],
              "caveats": null,
              "depends_on": {
              },
              "conflicts_with": null,
              "container": null,
              "auto_updates": true,
              "deprecated": false,
              "deprecation_date": null,
              "deprecation_reason": null,
              "disabled": false,
              "disable_date": null,
              "disable_reason": null,
              "tap_git_head": "bb54534136294da1751211f96a96ae7904879ffa",
              "languages": [
        
              ],
              "ruby_source_path": "Casks/u/unity-hub.rb",
              "ruby_source_checksum": {
                "sha256": "0c49a8c16590010638fdb6922f0f3e93674b8d12a336791af2b41933365c7d0c"
              }
            }
          ]
        }
        "#;
        let package_info: PackageInfoQuery = serde_json::from_str(json).unwrap();
        println!("{:?}", package_info);
    }
}
