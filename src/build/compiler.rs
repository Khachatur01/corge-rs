use crate::extension::Extension;
use crate::model::{Project, ProjectType};
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

#[derive(Clone)]
pub struct Compiler {
    project: PathBuf,
    include: PathBuf,
    output: PathBuf,

    compiler: String,
    archiver: String,
    compiler_flags: Vec<String>,
    linker_flags: Vec<String>,
}

impl Compiler {
    pub fn new(
        project: PathBuf,
        include: PathBuf,
        output: PathBuf,

        compiler: String,
        archiver: String,
        compiler_flags: Vec<String>,
        linker_flags: Vec<String>
    ) -> Self {
        Self {
            project,
            include,
            output,

            compiler,
            archiver,
            compiler_flags,
            linker_flags,
        }
    }

    pub fn compile(&self, project: &Project) {
        let source = Compiler::find_source_files(&self.project);

        /* todo: release */
        let cache = self.output.join("release").join("cache");
        fs::create_dir_all(&cache).unwrap();

        self.compile_sources(&source, &cache, &self.include, false);
        let objects: Vec<PathBuf> = fs::read_dir(cache).unwrap().map(|file| file.unwrap().path()).collect();

        match project.project_type {
            ProjectType::Executable => {
                self.generate_executable(&objects, &self.output.join("release"), &project.name);
            }
            ProjectType::StaticLibrary => {
                self.generate_static(&objects, &self.output.join("release"), &project.name);
            }
            ProjectType::DynamicLibrary => {
                self.generate_dynamic(&objects, &self.output.join("release"), &project.name);
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
