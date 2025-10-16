use anyhow::Result;
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::cli::BuildToolchain;

#[derive(Serialize, Deserialize, Subcommand, Debug, Default, Clone)]
pub enum LinkStrategy {
    #[default]
    Executable,
    StaticLibrary,
    DynamicLibrary,
}

impl LinkStrategy {
    pub fn to_yaml_tag(&self) -> String {
        match self {
            LinkStrategy::Executable => "!Executable".to_string(),
            LinkStrategy::StaticLibrary => "!StaticLibrary".to_string(),
            LinkStrategy::DynamicLibrary => "!DynamicLibrary".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub link_strategy: LinkStrategy,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OptimizationLevel {
    None,
    O,
    O1,
    O2,
    O3,
    O0,
    Os,
    Ofast,
    Og,
    Oz,
}

impl OptimizationLevel {
    pub fn as_gcc_flag(&self) -> Option<&str> {
        match self {
            OptimizationLevel::None => None,
            OptimizationLevel::O => Some("-O"),
            OptimizationLevel::O1 => Some("-O1"),
            OptimizationLevel::O2 => Some("-O2"),
            OptimizationLevel::O3 => Some("-O3"),
            OptimizationLevel::O0 => Some("-O0"),
            OptimizationLevel::Os => Some("-Os"),
            OptimizationLevel::Ofast => Some("-Ofast"),
            OptimizationLevel::Og => Some("-Og"),
            OptimizationLevel::Oz => Some("-Oz"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    /// https://gcc.gnu.org/onlinedocs/gcc/Optimize-Options.html
    pub optimization_level: OptimizationLevel,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Profiles {
    pub release: Option<Profile>,
    pub development: Option<Profile>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Registry {
    Git {
        url: String,
        branch: String,
    },
    FileSystem(String),
}


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Dependency {
    pub name: String,
    pub registry_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Toolchain {
    pub compiler: String,
    pub archiver: String,
    pub compiler_flags: Vec<String>,
    pub linker_flags: Vec<String>,
}
impl Default for Toolchain {
    fn default() -> Self {
        Self {
            compiler: "gcc".to_string(),
            archiver: "ar".to_string(),
            compiler_flags: vec![],
            linker_flags: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub project: Project,
    #[serde(default)]
    pub profiles: Profiles,
    #[serde(default)]
    pub registries: HashMap<String, Registry>,
    #[serde(default)]
    pub dependencies: Vec<Dependency>,
    #[serde(default)]
    pub toolchains: HashMap<String, Toolchain>
}

impl Config {
    /** Returns the toolchain name on an error case */
    pub fn toolchain(&self, toolchain: Option<BuildToolchain>) -> Result<(String, Toolchain)> {
        // let toolchain = toolchain.map(|toolchain| toolchain.0);

        match toolchain {
            None | Some(BuildToolchain::Default) => {
                let name = "default".to_string();
                let toolchain = Toolchain::default();

                Ok((name, toolchain))
            }
            Some(BuildToolchain::Named { name }) => {
                let toolchain = self.toolchains
                    .get(&name)
                    .cloned()
                    .ok_or_else(|| anyhow::anyhow!("Toolchain '{}' not found", name))?;

                Ok((name, toolchain))
            }
            Some(BuildToolchain::Custom { compiler, archiver, compiler_flags, linker_flags }) => {
                let name = "custom".to_string();
                let toolchain = Toolchain {
                    compiler,
                    archiver,
                    compiler_flags,
                    linker_flags,
                };

                Ok((name, toolchain))
            }
        }
    }
}
