use crate::cli::BuildModeCli;
use crate::config::{Profile, ProjectType, Toolchain};
use crate::extension::Extension;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};
use std::process::Command;

fn hash<P: AsRef<Path> + Hash>(path: P) -> String {
    /* todo: implement a logic to read source file concatenate it with its path then hash it */
    let mut hasher = DefaultHasher::new();

    path.hash(&mut hasher);

    hasher.finish().to_string()
}

fn add_flags(command: &mut Command, flags: &[String]) {
    for flag in flags {
        command.arg(flag);
    }
}

/* todo: use builder pattern */
#[derive(Clone)]
pub struct Compiler {
    project_dir: PathBuf,
    include_dir: PathBuf,
    output_dir: PathBuf,

    project_name: String,
    project_type: ProjectType,

    toolchain: Toolchain,
}

impl Compiler {
    pub fn new(
        project_dir: PathBuf,
        include_dir: PathBuf,
        output_dir: PathBuf,

        project_name: String,
        project_type: ProjectType,

        toolchain: Toolchain,
    ) -> Self {
        Self {
            project_dir,
            include_dir,
            output_dir,

            project_name,
            project_type,

            toolchain,
        }
    }

    pub fn compile(&self, build_mode: &BuildModeCli, profile: &Profile) {
        let source = Compiler::find_source_files(&self.project_dir);

        let build_dir = self.output_dir.join(build_mode.to_string());

        let cache_dir = build_dir.join("cache");
        fs::create_dir_all(&cache_dir).unwrap();

        let pic: bool = matches!(self.project_type, ProjectType::DynamicLibrary);

        self.compile_sources(&source, &cache_dir, &self.include_dir, pic, profile, &self.toolchain.compiler_flags);
        let objects: Vec<PathBuf> = fs::read_dir(cache_dir).unwrap().map(|file| file.unwrap().path()).collect();

        match self.project_type {
            ProjectType::Executable => {
                self.generate_executable(&objects, &build_dir, &self.project_name, &self.toolchain.linker_flags);
            }
            ProjectType::StaticLibrary => {
                self.generate_static(&objects, &build_dir, &self.project_name);
            }
            ProjectType::DynamicLibrary => {
                self.generate_dynamic(&objects, &build_dir, &self.project_name, &self.toolchain.linker_flags);
            }
        }
    }

    fn generate_executable(&self, objects: &[PathBuf], output: &PathBuf, output_name: &str, linker_flags: &[String]) {
        let output_name = Extension::Executable.file_name(&output_name, &self.toolchain.compiler);

        let mut command = Command::new(&self.toolchain.compiler);

        command
            .arg("-o")
            .arg(output.join(output_name));

        for object in objects {
            command.arg(object);
        }

        add_flags(&mut command, linker_flags);

        println!("Creating executable {:#?}", objects);
        println!("{:?}", command);
        command.output().unwrap();
    }

    fn generate_static(&self, objects: &[PathBuf], output: &PathBuf, output_name: &str) {
        let output_name = Extension::StaticLibrary.file_name(&output_name, &self.toolchain.compiler);

        let mut command = Command::new(&self.toolchain.archiver);
        command.arg("rcs");

        command.arg(output.join(output_name));

        for object in objects {
            command.arg(object);
        }

        println!("Creating static library {:#?}", objects);
        println!("{:?}", command);
        command.output().unwrap();
    }

    fn generate_dynamic(&self, objects: &[PathBuf], output: &PathBuf, output_name: &str, linker_flags: &[String]) {
        let output_name = Extension::DynamicLibrary.file_name(&output_name, &self.toolchain.compiler);

        let mut command = Command::new(&self.toolchain.compiler);
        command.arg("-shared");

        command
            .arg("-o")
            .arg(output.join(output_name));

        add_flags(&mut command, linker_flags);

        for object in objects {
            command.arg(object);
        }

        println!("Creating dynamic library {:#?}", objects);
        println!("{:?}", command);
        command.output().unwrap();
    }

    /**
     @param: sources - list of source files
     @param: output - output directory
     @param: pic - position independent code
     */
    fn compile_sources(&self, sources: &[PathBuf], output: &PathBuf, include: &PathBuf, pic: bool, profile: &Profile, compiler_flags: &[String]) {
        for source in sources {
            let output_name = Extension::Object.file_name(&hash(source), &self.toolchain.compiler);

            let output_file = output.join(output_name);

            let mut command = Command::new(&self.toolchain.compiler);

            if let Some(level) = profile.optimization_level.as_gcc_flag() {
                command.arg(level);
            }

            command
                .arg("-I")
                .arg(include);

            command
                .arg("-o")
                .arg(&output_file);

            command
                .arg("-c")
                .arg(source);

            add_flags(&mut command, &compiler_flags);

            if pic {
                command.arg("-fPIC");
            }

            println!("Compiling {:?} to {:?}", source, output_file);
            println!("{:?}", command);
            command.output().unwrap();
        }
    }

    fn find_source_files(directory: &Path) -> Vec<PathBuf> {
        let mut files = Vec::new();

        for file in fs::read_dir(directory).unwrap() {
            let file = file.unwrap();

            if file.path().is_dir() {
                let inner_files = Compiler::find_source_files(file.path().as_path());
                files.extend(inner_files);
            } else if let Some(extension) = file.path().extension() && extension == "c" {
                files.push(file.path());
            }
        }

        files
    }
}
