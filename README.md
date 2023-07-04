<h2 align="center"><img align="center" src="https://github.com/vrmiguel/vrmiguel/assets/36349314/5170cc9d-e6bf-4e47-a7c0-e02c8778b8ec" height="70px" />  guana </h2>

`guana` is the `Grpc User AgeNt Auth`.

## Project structure

In order to be modular, this project is structured as follows:

* `proto`
    - The Protobuf schema for the project.
* `guana-grpc-types`
    - Shared crate for the client and server defined in the [Protobuf schema](https://todo). This is the crate the compiles the `.proto` file and generates the [`prost`]() code to interface with it.
* `guana-server`
    - The library that implements the `UserAgentAnalyzer` gRPC service. This is useful if you'd like to embed Guana in another Rust project. See its README for more information.
* `guana`
    - The binary that runs `guana-server`, a highly performant gRPC server. This is useful if you want to run Guana standalone.

## Building

This project can be built using a [Nix shell](https://nixos.wiki/wiki/Development_environment_with_nix-shell), which provides a consistent and reproducible development by managing dependencies.

Once Nix is installed, run `nix-shell` in the root folder to spawn a subshell with the pre-configured development environment.

### Setting up Rust

Check if a working Rust toolchain is installed:

The Nix shell provides [Rustup](https://rustup.rs/), the Rust toolchain installer. To verify if you have a working Rust toolchain installed, run the following commands:

```bash
rustc --version # rustc 1.70.0 (90c541806 2023-05-31)
cargo --version # cargo 1.70.0 (ec8a8a0ca 2023-04-25)
```

If either of the two commands above is not found, ensure that the stable toolchain is installed by running:

```bash
rustup toolchain install stable
```