use serde;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageVersion {
    pub stable: String,
    pub head: String,
    pub bottle: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub full_name: String,
    pub oldnames: Vec<String>,
    pub aliases: Vec<String>,
    pub versions: PackageVersion,
    pub desc: String,
    pub license: String,
    pub homepage: String,
}

#[cfg(test)]
mod test {
    use crate::homebrew::package_info_model::PackageInfo;

    #[test]
    fn test_package_info() {
        let json = r#"
        [
            {
              "name": "ffmpeg",
              "full_name": "ffmpeg",
              "tap": "homebrew/core",
              "oldname": null,
              "oldnames": [
          
              ],
              "aliases": [
                "ffmpeg@6"
              ],
              "versioned_formulae": [
                "ffmpeg@5",
                "ffmpeg@4",
                "ffmpeg@2.8"
              ],
              "desc": "Play, record, convert, and stream audio and video",
              "license": "GPL-2.0-or-later",
              "homepage": "https://ffmpeg.org/",
              "versions": {
                "stable": "6.1.1",
                "head": "HEAD",
                "bottle": true
              },
              "urls": {
                "stable": {
                  "url": "https://ffmpeg.org/releases/ffmpeg-6.1.1.tar.xz",
                  "tag": null,
                  "revision": null,
                  "using": null,
                  "checksum": "8684f4b00f94b85461884c3719382f1261f0d9eb3d59640a1f4ac0873616f968"
                },
                "head": {
                  "url": "https://github.com/FFmpeg/FFmpeg.git",
                  "branch": "master",
                  "using": null
                }
              },
              "revision": 3,
              "version_scheme": 0,
              "bottle": {
                "stable": {
                  "rebuild": 0,
                  "root_url": "https://ghcr.io/v2/homebrew/core",
                  "files": {
                    "arm64_sonoma": {
                      "cellar": "/opt/homebrew/Cellar",
                      "url": "https://ghcr.io/v2/homebrew/core/ffmpeg/blobs/sha256:56f1280cb31c1958cb015d4fc8e9e813a6e4ef1bf3356e21e675758af157a96f",
                      "sha256": "56f1280cb31c1958cb015d4fc8e9e813a6e4ef1bf3356e21e675758af157a96f"
                    },
                    "arm64_ventura": {
                      "cellar": "/opt/homebrew/Cellar",
                      "url": "https://ghcr.io/v2/homebrew/core/ffmpeg/blobs/sha256:ac8c0cd2a6c230f43ccae2b10471161ccafc4d1779dd237b7fb8d1b480c7e914",
                      "sha256": "ac8c0cd2a6c230f43ccae2b10471161ccafc4d1779dd237b7fb8d1b480c7e914"
                    },
                    "arm64_monterey": {
                      "cellar": "/opt/homebrew/Cellar",
                      "url": "https://ghcr.io/v2/homebrew/core/ffmpeg/blobs/sha256:c3b81f61e2c7618c208c047253f1f866a403dea0dc3373f5454b600509fdd4c9",
                      "sha256": "c3b81f61e2c7618c208c047253f1f866a403dea0dc3373f5454b600509fdd4c9"
                    },
                    "sonoma": {
                      "cellar": "/usr/local/Cellar",
                      "url": "https://ghcr.io/v2/homebrew/core/ffmpeg/blobs/sha256:38af3c855780f2597f5532cce028da9165b633d707fc9aa415bf843b94ce4023",
                      "sha256": "38af3c855780f2597f5532cce028da9165b633d707fc9aa415bf843b94ce4023"
                    },
                    "ventura": {
                      "cellar": "/usr/local/Cellar",
                      "url": "https://ghcr.io/v2/homebrew/core/ffmpeg/blobs/sha256:b909984563e9089391c675937edea33f0099935b55477763065b17ab5b3c6075",
                      "sha256": "b909984563e9089391c675937edea33f0099935b55477763065b17ab5b3c6075"
                    },
                    "monterey": {
                      "cellar": "/usr/local/Cellar",
                      "url": "https://ghcr.io/v2/homebrew/core/ffmpeg/blobs/sha256:3b5705fb8736151c8c18a74d6cc55333629c83ea138b3428113a24827e427ce2",
                      "sha256": "3b5705fb8736151c8c18a74d6cc55333629c83ea138b3428113a24827e427ce2"
                    },
                    "x86_64_linux": {
                      "cellar": "/home/linuxbrew/.linuxbrew/Cellar",
                      "url": "https://ghcr.io/v2/homebrew/core/ffmpeg/blobs/sha256:f83ead85e924d8680b43df44a7b826abd13944ccfb09e5beda6d9a053b44a7ca",
                      "sha256": "f83ead85e924d8680b43df44a7b826abd13944ccfb09e5beda6d9a053b44a7ca"
                    }
                  }
                }
              },
              "pour_bottle_only_if": null,
              "keg_only": false,
              "keg_only_reason": null,
              "options": [
          
              ],
              "build_dependencies": [
                "pkg-config"
              ],
              "dependencies": [
                "aom",
                "aribb24",
                "dav1d",
                "fontconfig",
                "freetype",
                "frei0r",
                "gnutls",
                "harfbuzz",
                "jpeg-xl",
                "lame",
                "libass",
                "libbluray",
                "librist",
                "libsoxr",
                "libssh",
                "libvidstab",
                "libvmaf",
                "libvorbis",
                "libvpx",
                "opencore-amr",
                "openjpeg",
                "openvino",
                "opus",
                "rav1e",
                "rubberband",
                "sdl2",
                "snappy",
                "speex",
                "srt",
                "svt-av1",
                "tesseract",
                "theora",
                "webp",
                "x264",
                "x265",
                "xvid",
                "xz",
                "zeromq",
                "zimg"
              ],
              "test_dependencies": [
          
              ],
              "recommended_dependencies": [
          
              ],
              "optional_dependencies": [
          
              ],
              "uses_from_macos": [
                "bzip2",
                "libxml2",
                "zlib"
              ],
              "uses_from_macos_bounds": [
                {
                },
                {
                },
                {
                }
              ],
              "requirements": [
          
              ],
              "conflicts_with": [
          
              ],
              "conflicts_with_reasons": [
          
              ],
              "link_overwrite": [
          
              ],
              "caveats": null,
              "installed": [
          
              ],
              "linked_keg": null,
              "pinned": false,
              "outdated": false,
              "deprecated": false,
              "deprecation_date": null,
              "deprecation_reason": null,
              "disabled": false,
              "disable_date": null,
              "disable_reason": null,
              "post_install_defined": false,
              "service": null,
              "tap_git_head": "4211f998072ca8238f2448cff4d1e874a1853c4c",
              "ruby_source_path": "Formula/f/ffmpeg.rb",
              "ruby_source_checksum": {
                "sha256": "946626e41ad7172bfb59930072df88bbf5d2116b3cb59d936194ba6040fc5f0b"
              }
            }
          ]          
        "#;
        let package_info: Vec<PackageInfo> = serde_json::from_str(json).unwrap();
        println!("{:?}", package_info);
    }
}

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
