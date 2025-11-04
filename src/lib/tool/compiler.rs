use crate::config::{Profile, Toolchain};
use crate::extension_manager::Extension;
use crate::std_command_ext::ExecuteCommand;
use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fmt::Debug;
use std::fs;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::process::Command;

fn hash<P: AsRef<Path> + Hash + Debug>(path: P) -> Result<String> {
    let source_file_name = path
        .as_ref()
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("Failed to get file name from {:?}", path))?;

    let content = fs::read_to_string(&path)?;
    let file = format!("{:?}-{}", path, content);

    let mut hasher = Sha256::new();

    hasher.update(file);

    let hash_bytes = hasher.finalize();
    let hash: String = hash_bytes
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect();

    Ok(format!("{}.{}", source_file_name.to_string_lossy(), hash))
}

pub struct Compiler {
    profile: Profile,
    toolchain: Toolchain,
    include_path: PathBuf,
}

impl Compiler {
    pub fn new(profile: Profile, toolchain: Toolchain, include_path: PathBuf) -> Self {
        Self {
            profile,
            toolchain,
            include_path,
        }
    }

    /**
        @param: source_files - list of source files
        @param: output_path - output directory path
        @param: pic - position independent code

        @return: list of compiled object files
    */
    pub fn compile(&self, source_files: &[PathBuf], output_path: &PathBuf, pic: bool) -> Result<Vec<PathBuf>> {
        let mut object_files = vec![];

        for source_file in source_files {
            let output_stem = hash(source_file)
                .with_context(|| format!("Failed to hash source file {:?}", source_file))?;

            let output_name = Extension::Object.file_name(&output_stem, &self.toolchain.compiler);

            let output_file = output_path.join(output_name);

            object_files.push(output_file.clone());

            let file_exists = fs::exists(&output_file).with_context(|| format!("Failed to check if file exists {:?}", output_file))?;
            if file_exists {
                log::info!("Skipping already compiled file {:?}", source_file);
                continue;
            }
            log::info!("Compiling {:?} into {}", source_file, output_path.display());

            let mut command = Command::new(&self.toolchain.compiler);

            if let Some(level) = self.profile.optimization_level.as_gcc_flag() {
                command.arg(level);
            }

            command
                .arg("-I")
                .arg(&self.include_path);

            command.args(&self.toolchain.compiler_flags);

            if pic {
                command.arg("-fPIC");
            }

            command
                .arg("-c")
                .arg(source_file);

            command
                .arg("-o")
                .arg(&output_file);

            command.execute(true)
                .with_context(|| format!("Failed to compile file {:?}", source_file))?;
        }

        Ok(object_files)
    }
}
