use std::path::PathBuf;
use std::process::Command;
use anyhow::Result;
use crate::config::{LinkStrategy, Toolchain};
use crate::extension_manager::Extension;
use crate::std_command_ext::ExecuteCommand;

pub struct Linker {
    toolchain: Toolchain,
}

impl Linker {
    pub fn new(toolchain: Toolchain) -> Self {
        Self {
            toolchain,
        }
    }

    pub fn link(&self, link_strategy: &LinkStrategy, object_files: &[PathBuf], output_path: &PathBuf, output_name: &str) -> Result<()> {
        let mut command = match link_strategy {
            LinkStrategy::Executable => {
                let mut command = Command::new(&self.toolchain.compiler);

                let output_name = Extension::Executable.file_name(output_name, &self.toolchain.compiler);
                command
                    .arg("-o")
                    .arg(output_path.join(output_name));

                command
            }
            LinkStrategy::StaticLibrary => {
                let mut command = Command::new(&self.toolchain.archiver);
                command.arg("rcs");

                let output_name = Extension::StaticLibrary.file_name(output_name, &self.toolchain.compiler);
                command
                    .arg(output_path.join(output_name));

                command
            }
            LinkStrategy::DynamicLibrary => {
                let mut command = Command::new(&self.toolchain.compiler);
                command.arg("-shared");

                let output_name = Extension::DynamicLibrary.file_name(output_name, &self.toolchain.compiler);
                command
                    .arg("-o")
                    .arg(output_path.join(output_name));

                command
            }
        };

        command.args(&self.toolchain.linker_flags);

        for object_file in object_files {
            command.arg(object_file);
        }

        command.execute(true)?;

        Ok(())
    }
}
