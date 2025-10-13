# corge-rs — A simple C build tool (C++ support planned)

corge-rs is a small build system written in Rust that helps you initialize, build, and manage dependencies for C projects
(using YAML configuration). It supports build profiles, pluggable toolchains, and multiple link strategies.

Note: The CLI subcommand `run` is not implemented yet.

## Stack and entry points
- Language: Rust (edition 2024)
- Package manager: Cargo
- CLI framework: clap (derive)
- Config: serde + serde_yaml
- Logging: log + simple_logger
- Binary entry point: src/main.rs (parses CLI and dispatches to subcommands)

## Features
- Initialize a C project skeleton with a build.yaml and src/main.c
- Multiple build profiles (development/release)
- GCC optimization level configuration per profile
- Link strategies: executable, static library, dynamic library
- Dependency management via registries (git and filesystem)
- Customizable toolchains (compiler, archiver, flags)
- Cross-platform output file naming via target-lexicon
- Planned: C++ support and `run` command

## Requirements
- Rust toolchain (stable) and Cargo
  - rust-toolchain.toml pins channel = "stable"
- A C toolchain for building your C projects:
  - Default toolchain uses system gcc and ar
  - Custom toolchains can be declared in build.yaml
- Git (optional; required if you use a Git registry for dependencies)

## Installation
- Local install from this repo:
  - cargo install --path .
- Or run without installing:
  - cargo run -- <subcommand> [options]

TODO: If this is published to crates.io, add the crate name and cargo install corge-rs instructions here.

## Quick start
Initialize a new C project in ./my_app as an executable (default):
- corge-rs init ./my_app

Initialize as a static or dynamic library:
- corge-rs init ./my_lib --s-lib
- corge-rs init ./my_dylib --d-lib

Build the project (from the project directory that contains build.yaml):
- corge-rs build .
- corge-rs build . --release
- corge-rs build . --toolchain pic24

Clean build outputs (and optionally dependency cache):
- corge-rs clean .
- corge-rs clean . --deps-too

Note: The run subcommand exists but is not implemented yet.

## CLI reference
Subcommands and key options:
- init [PATH] [--executable | --s-lib | --d-lib]
  - Creates: src/main.c, build.yaml, .gitignore (if the directory is empty)
  - Defaults to Executable if none of the flags are passed
- build [PATH] [--dev | --release] [--toolchain <NAME>]
  - Parses build.yaml, fetches dependencies, compiles sources, links outputs
  - Default build mode: development
  - Default toolchain: implicit "default" (gcc/ar with no extra flags)
- clean [PATH] [--deps-too]
  - Removes target/ (and dependency/ if --deps-too)
- run [PATH]
  - TODO: Not implemented yet

## Configuration (build.yaml)
Top-level structure (see projects/example_app/build.yaml for a full example):
- project:
  - name: string
  - version: string
  - link_strategy: one of !Executable | !StaticLibrary | !DynamicLibrary
- profiles:
  - release.development: optimization_level: one of [None, O, O0, O1, O2, O3, Os, Og, Oz, Ofast]
- registries: map of <name> -> registry
  - !Git: { url: string, branch: string }
  - !FileSystem: string (path to a folder containing dependency projects)
- dependencies: list of { name: string, registry_name: string }
- toolchains: map of <name> -> { compiler, archiver, compiler_flags: [], linker_flags: [] }

Example:
```yaml
project:
  name: my-app
  version: 1.0.0
  link_strategy: !Executable

profiles:
  release:
    optimization_level: O
  development:
    optimization_level: None

registries:
  filesystem: !FileSystem ../
  github: !Git
    url: https://github.com/my-corge-repo
    branch: master

dependencies:
- name: lib1
  registry_name: filesystem

toolchains:
  pic24:
    compiler: /opt/microchip/xc16/v2.10/bin/bin/elf-gcc
    archiver: /opt/microchip/xc16/v2.10/bin/bin/elf-ar
    compiler_flags:
      - "-Wall"
    linker_flags: []
```

## Build outputs and directories
When building, the following directories are created under your project:
- dependency/
  - source/: fetched dependency sources (nested by dependency name)
  - include/: copied header files from dependencies (mirrors their src/ structure)
- target/<build_mode>/<toolchain>/
  - cache/project/: compiled object files for your project
  - cache/dependency/: compiled object files for dependencies
  - output/: final artifacts
    - Executable: <name> (with platform-specific extension)
    - Static library: lib<name>.a|.lib
    - Dynamic library: lib<name>.so|.dylib|.dll

Note on IDE/include paths
- If you use an IDE (e.g., CLion, VS Code, etc.), add the dependency/include directory of your project as an "Include Directory" so headers from dependencies are discovered by code completion and standalone compilation.
- Alternatively, ensure your compiler flags include -I dependency/include (or the absolute path to that folder) when building outside of corge-rs.

## Environment variables
- Logging is provided by simple_logger and can be configured via environment variables.
  - Common usage: set RUST_LOG to control verbosity, e.g. RUST_LOG=info or RUST_LOG=trace
  - TODO: Confirm the exact environment variable names supported by simple_logger::SimpleLogger::env()

## Scripts
This project doesn’t use custom shell scripts. Use Cargo and the built-in CLI:
- cargo build, cargo run, and cargo install
- cargo run -- <subcommand> [options]

## Project structure (repo)
- src/: Rust sources for the CLI and build tool
  - cli.rs, main.rs, command/*, tool/*
- projects/example_app: sample C project and build.yaml
- projects/lib1, projects/lib2: example dependencies
- rust-toolchain.toml: pins Rust channel/targets
- Cargo.toml, Cargo.lock
