pub struct Manifest {
    name: String,
    link_strategy: LinkStrategy,
    sources: Vec<PathBuf>,
}

pub get_uncompiled_dependencies(project_dir: &PathBuf) -> Vec<Manifest> {
    vec![]
}
