# Cargo
cargo是一个rust项目构建以及包管理工具

- 新建项目
```
cargo new hello_world
```

初始化一个名字叫 hello_world 的目录。里面包含一个目录 src 以及一个文件 Cargo.toml，src 目录中包含一个 main.rs 文件。

项目中同时会初始化 git 仓库。

- 编译项目

```
cargo build 
```
编译速度更快，运行速度慢。未编译优化，构建完文件生成于 target/debug 目录
```
cargo build --release 
```
编译速度慢，运行速度快。编译过程中会进行优化，构建完文件生成于 target/release 目录

## Cargo.toml 文件结构

* [`cargo-features`](unstable.md) --- Unstable, nightly-only features.
* [`[package]`](#the-package-section) --- Defines a package.
  * [`name`](#the-name-field) --- The name of the package.
  * [`version`](#the-version-field) --- The version of the package.
  * [`authors`](#the-authors-field) --- The authors of the package.
  * [`edition`](#the-edition-field) --- The Rust edition.
  * [`rust-version`](#the-rust-version-field) --- The minimal supported Rust version.
  * [`description`](#the-description-field) --- A description of the package.
  * [`documentation`](#the-documentation-field) --- URL of the package documentation.
  * [`readme`](#the-readme-field) --- Path to the package's README file.
  * [`homepage`](#the-homepage-field) --- URL of the package homepage.
  * [`repository`](#the-repository-field) --- URL of the package source repository.
  * [`license`](#the-license-and-license-file-fields) --- The package license.
  * [`license-file`](#the-license-and-license-file-fields) --- Path to the text of the license.
  * [`keywords`](#the-keywords-field) --- Keywords for the package.
  * [`categories`](#the-categories-field) --- Categories of the package.
  * [`workspace`](#the-workspace-field) --- Path to the workspace for the package.
  * [`build`](#the-build-field) --- Path to the package build script.
  * [`links`](#the-links-field) --- Name of the native library the package links with.
  * [`exclude`](#the-exclude-and-include-fields) --- Files to exclude when publishing.
  * [`include`](#the-exclude-and-include-fields) --- Files to include when publishing.
  * [`publish`](#the-publish-field) --- Can be used to prevent publishing the package.
  * [`metadata`](#the-metadata-table) --- Extra settings for external tools.
  * [`default-run`](#the-default-run-field) --- The default binary to run by [`cargo run`].
  * [`autobins`](cargo-targets.md#target-auto-discovery) --- Disables binary auto discovery.
  * [`autoexamples`](cargo-targets.md#target-auto-discovery) --- Disables example auto discovery.
  * [`autotests`](cargo-targets.md#target-auto-discovery) --- Disables test auto discovery.
  * [`autobenches`](cargo-targets.md#target-auto-discovery) --- Disables bench auto discovery.
  * [`resolver`](resolver.md#resolver-versions) --- Sets the dependency resolver to use.
* Target tables: (see [configuration](cargo-targets.md#configuring-a-target) for settings)
  * [`[lib]`](cargo-targets.md#library) --- Library target settings.
  * [`[[bin]]`](cargo-targets.md#binaries) --- Binary target settings.
  * [`[[example]]`](cargo-targets.md#examples) --- Example target settings.
  * [`[[test]]`](cargo-targets.md#tests) --- Test target settings.
  * [`[[bench]]`](cargo-targets.md#benchmarks) --- Benchmark target settings.
* Dependency tables:
  * [`[dependencies]`](specifying-dependencies.md) --- Package library dependencies.
  * [`[dev-dependencies]`](specifying-dependencies.md#development-dependencies) --- Dependencies for examples, tests, and benchmarks.
  * [`[build-dependencies]`](specifying-dependencies.md#build-dependencies) --- Dependencies for build scripts.
  * [`[target]`](specifying-dependencies.md#platform-specific-dependencies) --- Platform-specific dependencies.
* [`[badges]`](#the-badges-section) --- Badges to display on a registry.
* [`[features]`](features.md) --- Conditional compilation features.
* [`[lints]`](#the-lints-section) --- Configure linters for this package.
* [`[patch]`](overriding-dependencies.md#the-patch-section) --- Override dependencies.
* [`[replace]`](overriding-dependencies.md#the-replace-section) --- Override dependencies (deprecated).
* [`[profile]`](profiles.md) --- Compiler settings and optimizations.
* [`[workspace]`](workspaces.md) --- The workspace definition.
