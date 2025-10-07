use std::fs;
use crate::cli::InitArgs;

pub fn init(init_args: InitArgs) {
    println!("Initializing project in directory {:?}", init_args.path);
    if fs::exists(&init_args.path).unwrap() {
        panic!("Directory {:?} already exists", init_args.path);
    }

    fs::create_dir_all(&init_args.path).unwrap();

    fs::create_dir_all(init_args.path.join("src")).unwrap();
    fs::write(init_args.path.join("src").join("main.c"), "").unwrap();
    fs::write(init_args.path.join(".gitignore"), "").unwrap();
    fs::write(init_args.path.join("build.yaml"), "").unwrap();
}
