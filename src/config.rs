use clap::Subcommand;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Subcommand, Debug, Default, Clone)]
pub enum ProjectType {
    #[default]
    #[serde(rename = "executable")]
    Executable,
    #[serde(rename = "static-library")]
    StaticLibrary,
    #[serde(rename = "dynamic-library")]
    DynamicLibrary,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Project {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub project_type: ProjectType,
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
pub enum Repository {
    Git {
        url: String,
        branch: String,
    },
    FileSystem(String),
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub repository_name: String,
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
    pub profiles: Option<Profiles>,
    pub repositories: Option<HashMap<String, Repository>>,
    pub dependencies: Option<Vec<Dependency>>,
    pub toolchains: Option<HashMap<String, Toolchain>>
}
