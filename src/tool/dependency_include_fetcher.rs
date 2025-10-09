use std::fs;
use anyhow::Result;
use std::path::PathBuf;
use crate::tool::dependency_source_fetcher::Artifact;

/**
    Converts a dependency tree into a flat dependency list.
 */
pub struct DependencyIncludeFetcher<'a> {
    artifacts: &'a [Artifact]
}

impl<'a> DependencyIncludeFetcher<'a> {
    pub fn new(artifacts: &'a [Artifact]) -> Self {
        Self {
            artifacts,
        }
    }

    /* Fetch dependencies header files */
    pub fn fetch(&self, include_dir: &PathBuf) -> Result<()> {
        for artifact in self.artifacts {
            copy_headers(&artifact.path.join("src"), &include_dir.join(&artifact.dependency.name))?;
        }

        Ok(())
    }
}


fn copy_headers(src_path: &PathBuf, dst_path: &PathBuf) -> Result<()> {
    let source_dir = fs::read_dir(src_path)?;

    for source_file in source_dir {
        let source_file = source_file?;

        if source_file.path().is_dir() {
            copy_headers(
                &source_file.path(),
                &dst_path.join(source_file.file_name())
            )?;
        } else {
            fs::create_dir_all(dst_path)?;

            let is_header_file = source_file
                .path()
                .extension()
                .map(|extension| extension == "h")
                .unwrap_or(false);

            if is_header_file {
                fs::copy(
                    source_file.path(),
                    dst_path.join(source_file.file_name())
                )?;
            }
        }
    }

    Ok(())
}
