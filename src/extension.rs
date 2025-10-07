use std::process::Command;
use std::str::FromStr;
use target_lexicon::{OperatingSystem, Triple};

pub enum Extension {
    Object,
    Executable,
    StaticLibrary,
    DynamicLibrary,
}

impl Extension {
    fn linux_extension(&self) -> Option<String> {
        match self {
            Extension::Object => Some("o".to_string()),
            Extension::Executable => None,
            Extension::StaticLibrary => Some("a".to_string()),
            Extension::DynamicLibrary => Some("so".to_string())
        }
    }
    fn macos_extension(&self) -> Option<String> {
        match self {
            Extension::Object => Some("o".to_string()),
            Extension::Executable => None,
            Extension::StaticLibrary => Some("a".to_string()),
            Extension::DynamicLibrary => Some("dylib".to_string())
        }
    }
    fn windows_extension(&self) -> Option<String> {
        match self {
            Extension::Object => Some("obj".to_string()),
            Extension::Executable => Some("exe".to_string()),
            Extension::StaticLibrary => Some("lib".to_string()),
            Extension::DynamicLibrary => Some("dll".to_string())
        }
    }

    fn for_triple(&self, target_triple: &str) -> Option<String> {
        let operating_system =
            match Triple::from_str(target_triple) {
                Ok(triple) => triple.operating_system,
                Err(_) => OperatingSystem::Linux
            };

        match operating_system {
            OperatingSystem::Linux => self.linux_extension(),
            OperatingSystem::MacOSX(_) => self.macos_extension(),
            OperatingSystem::Windows => self.windows_extension(),
            _ => self.linux_extension(),
        }
    }

    fn for_compiler(&self, compiler: &str) -> Option<String> {
        let stdout = Command::new(compiler).arg("-dumpmachine").output().unwrap().stdout;

        /* fixme */
        let triple = String::from_utf8(stdout).unwrap().strip_suffix("\n").unwrap().to_string();

        self.for_triple(&triple)
    }

    pub fn file_name(&self, name: &str, compiler: &str) -> String {
        let extension = match self.for_compiler(compiler) {
            None => "",
            Some(extension) => &format!(".{}", extension),
        };

        match self {
            Extension::Object => format!("{}{}", name, extension),
            Extension::Executable =>  format!("{}{}", name, extension),
            Extension::StaticLibrary => format!("lib{}{}", name, extension),
            Extension::DynamicLibrary => format!("lib{}{}", name, extension),
        }
    }
}
