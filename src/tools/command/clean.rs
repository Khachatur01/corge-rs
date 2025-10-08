use crate::cli::CleanArgs;
use std::fs;

pub fn clean(clean_args: CleanArgs) {
    fs::remove_dir_all(clean_args.path.join("target")).unwrap();

    if clean_args.deps_too {
        fs::remove_dir_all(clean_args.path.join("dependency")).unwrap();
    }
}
