use std::collections::HashMap;
use clap::Subcommand;
use serde::{Deserialize, Serialize};

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
    Fast,
    Full,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Builder {
    pub compiler: String,
    pub archiver: String,
    pub compiler_flags: Vec<String>,
    pub linker_flags: Vec<String>,
}
impl Default for Builder {
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
    pub builder: Option<Builder>,
}
