use crate::command_line_arguments::ProfileCli;
use crate::config::ProjectType;
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

/* todo: use builder pattern */
#[derive(Clone)]
pub struct Compiler {
    project_dir: PathBuf,
    include_dir: PathBuf,
    output_dir: PathBuf,

    project_name: String,
    project_type: ProjectType,

    compiler: String,
    archiver: String,
    compiler_flags: Vec<String>,
    linker_flags: Vec<String>,
}

impl Compiler {
    pub fn new(
        project_dir: PathBuf,
        include_dir: PathBuf,
        output_dir: PathBuf,

        project_name: String,
        project_type: ProjectType,

        compiler: String,
        archiver: String,
        compiler_flags: Vec<String>,
        linker_flags: Vec<String>
    ) -> Self {
        Self {
            project_dir,
            include_dir,
            output_dir,

            project_name,
            project_type,

            compiler,
            archiver,
            compiler_flags,
            linker_flags,
        }
    }

    pub fn compile(&self, profile_cli: &ProfileCli) {
        let source = Compiler::find_source_files(&self.project_dir);

        let profile_dir = self.output_dir.join(profile_cli.to_string());

        let cache_dir = profile_dir.join("cache");
        fs::create_dir_all(&cache_dir).unwrap();

        self.compile_sources(&source, &cache_dir, &self.include_dir, false);
        let objects: Vec<PathBuf> = fs::read_dir(cache_dir).unwrap().map(|file| file.unwrap().path()).collect();

        match self.project_type {
            ProjectType::Executable => {
                self.generate_executable(&objects, &profile_dir, &self.project_name);
            }
            ProjectType::StaticLibrary => {
                self.generate_static(&objects, &profile_dir, &self.project_name);
            }
            ProjectType::DynamicLibrary => {
                self.generate_dynamic(&objects, &profile_dir, &self.project_name);
            }
        }
    }

    fn generate_executable(&self, objects: &[PathBuf], output: &PathBuf, output_name: &str) {
        let output_name = Extension::Executable.file_name(&output_name, &self.compiler);

        let mut command = Command::new(&self.compiler);

        command
            .arg("-o")
            .arg(output.join(output_name));

        for object in objects {
            command.arg(object);
        }

        println!("Creating executable {:#?}", objects);
        println!("{:?}", command);
        command.output().unwrap();
    }

    fn generate_static(&self, objects: &[PathBuf], output: &PathBuf, output_name: &str) {
        let output_name = Extension::StaticLibrary.file_name(&output_name, &self.compiler);

        let mut command = Command::new(&self.archiver);
        command.arg("rcs");

        command.arg(output.join(output_name));

        for object in objects {
            command.arg(object);
        }

        println!("Creating static library {:#?}", objects);
        println!("{:?}", command);
        command.output().unwrap();
    }

    fn generate_dynamic(&self, objects: &[PathBuf], output: &PathBuf, output_name: &str) {
        let output_name = Extension::DynamicLibrary.file_name(&output_name, &self.compiler);

        let mut command = Command::new(&self.compiler);
        command.arg("-shared");

        command
            .arg("-o")
            .arg(output.join(output_name));

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
    fn compile_sources(&self, sources: &[PathBuf], output: &PathBuf, include: &PathBuf, pic: bool) {
        for source in sources {
            let mut output_file = output.join(hash(source));
            output_file.set_extension("o");

            let mut command = Command::new(&self.compiler);

            command
                .arg("-I")
                .arg(include);

            command
                .arg("-o")
                .arg(output_file);

            command
                .arg("-c")
                .arg(source);

            if pic {
                command.arg("-fPIC");
            }

            println!("Compiling {:?}", source);
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
