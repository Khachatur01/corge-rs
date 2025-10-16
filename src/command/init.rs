use crate::cli::InitArgs;
use anyhow::{Context, Result};
use std::fs;
use std::process::Command;
use crate::std_command_ext::ExecuteCommand;

const MAIN_C_CONTENT: &str = r###"
int main() {
    return 0;
}
"###;

const GITIGNORE_CONTENT: &str = r###"
/target
/dependency
/compilation_database
"###;

const BUILD_YAML_CONTENT: &str = r###"
project:
  name: {{name}}
  version: 1.0.0
  link_strategy: {{link_strategy}}

profiles:
  release:
    optimization_level: O
  development:
    optimization_level: None
"###;

pub fn init(init_args: InitArgs) -> Result<()> {
    let project_name = init_args.path.file_name().unwrap().to_str().unwrap();

    log::info!("Initializing project in directory {:?}", init_args.path);

    let directory_exists = fs::exists(&init_args.path)
        .with_context(|| format!("Failed to check if directory {:?} exists", init_args.path))?;

    if directory_exists {
        let read_dir = fs::read_dir(&init_args.path)
            .with_context(|| format!("Failed to read directory {:?}", &init_args.path))?;

        if read_dir.count() > 0 {
            return Err(anyhow::anyhow!("Directory {:?} is not empty", &init_args.path));
        }
    }

    let src_dir = init_args.path.join("src");
    fs::create_dir_all(&src_dir)
        .with_context(|| format!("Failed to create directory {:?}", &src_dir))?;

    let main_c_content = MAIN_C_CONTENT.trim_start();
    fs::write(&src_dir.join("main.c"), main_c_content)
        .with_context(|| format!("Failed to create file {:?}", &src_dir.join("main.c")))?;

    let link_strategy = init_args.link_strategy().to_yaml_tag();

    let build_yaml_content = BUILD_YAML_CONTENT
        .trim_start()
        .replace("{{name}}", project_name)
        .replace("{{link_strategy}}", &link_strategy);

    fs::write(init_args.path.join("build.yaml"), build_yaml_content)
        .with_context(|| format!("Failed to create file {:?}", &src_dir))?;

    if !init_args.no_git {
        Command::new("git")
            .arg("init")
            .current_dir(&init_args.path)
            .execute(true)
            .with_context(|| format!("Failed to initialize git repository in {:?}", &init_args.path))?;

        let gitignore_content = GITIGNORE_CONTENT.trim_start();
        fs::write(&init_args.path.join(".gitignore"), gitignore_content)
            .with_context(|| format!("Failed to create file {:?}", &src_dir))?;
    }

    log::info!("PROJECT SUCCESSFULLY INITIALIZED ({})", link_strategy);
    Ok(())
}
