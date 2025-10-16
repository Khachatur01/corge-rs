use std::fs;
use std::path::PathBuf;

fn fetch_files_by_extension(path: &PathBuf, extension: &str) -> anyhow::Result<Vec<PathBuf>> {
    let mut c_files = vec![];

    for dir_entry in fs::read_dir(path)? {
        let dir_entry = dir_entry?;

        if dir_entry.path().is_dir() {
            let child_c_files = fetch_files_by_extension(&dir_entry.path(), extension)?;
            c_files.extend(child_c_files);
        } else {
            let is_source_file = dir_entry
                .path()
                .extension()
                .map(|file_extension| file_extension == extension)
                .unwrap_or(false);

            if is_source_file {
                c_files.push(dir_entry.path());
            }
        }
    }

    Ok(c_files)
}

pub fn fetch_files(path: &PathBuf, extension: &str) -> anyhow::Result<Vec<PathBuf>> {
    let src_path = path.join("src");

    fetch_files_by_extension(&src_path, extension)
}
