use crate::config::Toolchain;

pub struct Linker {
    toolchain: Toolchain,
}

impl Linker {
    pub fn new(toolchain: Toolchain) -> Self {
        Self {
            toolchain,
        }
    }

    pub fn link(&self) -> anyhow::Result<()> {
        Ok(())
    }
}