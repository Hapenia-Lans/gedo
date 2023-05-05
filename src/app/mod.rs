/**
 * Copyright (c) 2023 hapenia
 *
 * This software is released under the MIT License.
 * https://opensource.org/licenses/MIT
 */
mod args;
mod error;

use std::{collections::HashMap, fmt::Display, path::PathBuf, str::FromStr};

use anyhow::{anyhow, Context};
pub use args::Args;
use dialoguer;
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct App {
    pub appconfig: AppConfig,
    pub manifest: Manifest,
    pub package_list: PackageList,
}

impl App {
    pub fn run(mut self, args: Args) -> anyhow::Result<()> {
        self.check_env()?;
        match args.command {
            args::Commands::Install { version } => {
                let version: GodotVersion = version.parse()?;
                let mut confirm_override = dialoguer::Confirm::new();
                confirm_override.with_prompt(format!(
                    "Version {} has already exists, replace it with new installation?",
                    &version
                ));
                let forced = self.manifest.has(&version) && confirm_override.interact()?;
                let info = self.download_godot(&version, forced)?;
                self.register_version(&info)
            }
            args::Commands::Set { version } => self.set_current(&version.parse()?),
            args::Commands::Update => self.refresh_package_list(),
            args::Commands::Upgrade => self.upgrade(),
        }
    }
    fn download_godot(
        &mut self,
        version: &GodotVersion,
        forced: bool,
    ) -> anyhow::Result<GodotInfo> {
        // 1. get url
        let url = self.package_list.url(version).with_context(|| {
            format!("version not found: `{}` not found in package list", version)
        })?;
        // 2. download into temp
        // 3. move temp file to version dir, if forced = true, remove previous installation
        todo!()
    }
    fn register_version(&mut self, info: &GodotInfo) -> anyhow::Result<()> {
        // 1. add version into manifest
        // 2. save manifest
        todo!()
    }
    fn check_env(&mut self) -> anyhow::Result<()> {
        // 检查是否存在 GODOT_BIN，如果不存在，则尝试设置；
        // 检查是否存在 GODOT，如果不存在，则尝试设置；
        todo!()
    }
    fn refresh_package_list(&mut self) -> anyhow::Result<()> {
        todo!()
    }
    fn set_current(&mut self, version: &GodotVersion) -> anyhow::Result<()> {
        // 1. set env var GODOT_BIN
        // 2. set env var GODOT
        todo!()
    }
    fn upgrade(&mut self) -> anyhow::Result<()> {
        todo!()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone)]
enum TestMark {
    Stable,
    Alpha(u8),
    Beta(u8),
    Rc(u8),
}

impl FromStr for TestMark {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(num) = s.strip_prefix("alpha") {
            let num = num.parse::<u8>()?;
            Ok(TestMark::Alpha(num))
        } else if let Some(num) = s.strip_prefix("beta") {
            let num = num.parse::<u8>()?;
            Ok(TestMark::Beta(num))
        } else if let Some(num) = s.strip_prefix("rc") {
            let num = num.parse::<u8>()?;
            Ok(TestMark::Rc(num))
        } else if s == "stable" {
            Ok(TestMark::Stable)
        } else {
            return Err(anyhow!("Invalid testmark {}", s));
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone)]
struct GodotVersion {
    major: u8,
    minor: u8,
    patch: u8,
    testmark: TestMark,
    is_mono: bool,
}

impl Display for GodotVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // examples:
        //  - v3.4.3-stable
        //  - v3.4.3-rc6_mono
        //  - v4.0.0_mono
        write!(
            f,
            "v{}.{}.{}{}{}",
            self.major,
            self.minor,
            self.patch,
            match self.testmark {
                TestMark::Alpha(x) => format!("-alpha{}", x),
                TestMark::Beta(x) => format!("-beta{}", x),
                TestMark::Rc(x) => format!("-rc{}", x),
                TestMark::Stable => format!("-stable"),
            },
            if self.is_mono { "_mono" } else { "" }
        )
    }
}
impl FromStr for GodotVersion {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = error::Error::CannotParseVersion(s.to_string());
        let mut vcs = s.split(['-', '_']);
        // expected:
        // vcs[0]: vN.N.N
        // vcs[1]: stable/alphaN/betaN/rcN
        // vcs[2]: mono or nothing
        let Some(version_num_str) = vcs.next() else {
            return Err(err);
        };
        let Some((Ok(major), Ok(minor), Ok(patch))) = version_num_str
            .strip_prefix("v")
            .and_then(|x| Some(x.split(".")))
            .and_then(|x| {
                let vec = x.collect::<Vec<_>>();
                if vec.len() != 3 {
                    return None;
                }
                let major = vec[0].parse::<u8>();
                let minor = vec[1].parse::<u8>();
                let patch = vec[2].parse::<u8>();
                Some((major, minor, patch))
            }) else { return Err(err) };
        let Some(testmark_str) = vcs.next() else {
            return Err(err);
        };
        let testmark = testmark_str.parse().map_err(|_| err)?;
        let is_mono = vcs.next().is_some();
        Ok(Self {
            major,
            minor,
            patch,
            testmark,
            is_mono,
        })
    }
}

#[derive(Default, Debug)]
pub struct AppConfig {}

impl AppConfig {
    pub fn load_or_new() -> anyhow::Result<Self> {
        todo!()
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Manifest {
    installed_apps: HashMap<GodotVersion, GodotInfo>,
    current_app: Option<GodotVersion>,
}

impl Manifest {
    pub fn load_or_new() -> anyhow::Result<Self> {
        todo!()
    }
    fn has(&self, version: &GodotVersion) -> bool {
        self.installed_apps.contains_key(version)
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PackageList {
    prefix_url: String,
    win32: HashMap<GodotVersion, String>,
    win64: HashMap<GodotVersion, String>,
    linux_x86_32: HashMap<GodotVersion, String>,
    linux_x86_64: HashMap<GodotVersion, String>,
    macos: HashMap<GodotVersion, String>,
}

impl PackageList {
    pub fn load_or_new() -> anyhow::Result<Self> {
        todo!()
    }
    fn url(&self, version: &GodotVersion) -> Option<Url> {
        todo!()
    }
}

#[derive(Serialize, Debug, Deserialize)]
struct GodotInfo {
    version: GodotVersion,
    bin_path: PathBuf,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_godot_version() {
        let should_success = [
            (
                "v3.3.3-stable",
                GodotVersion {
                    major: 3,
                    minor: 3,
                    patch: 3,
                    testmark: TestMark::Stable,
                    is_mono: false,
                },
            ),
            (
                "v3.4.3-stable_mono",
                GodotVersion {
                    major: 3,
                    minor: 4,
                    patch: 3,
                    testmark: TestMark::Stable,
                    is_mono: true,
                },
            ),
            (
                "v4.0.1-alpha1",
                GodotVersion {
                    major: 4,
                    minor: 0,
                    patch: 1,
                    testmark: TestMark::Alpha(1),
                    is_mono: false,
                },
            ),
        ];
        for (str, vcs) in should_success {
            assert_eq!(str.parse::<GodotVersion>().unwrap(), vcs)
        }
    }
}
