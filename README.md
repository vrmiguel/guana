<h2 align="center"><img align="center" src="https://github.com/vrmiguel/vrmiguel/assets/36349314/5170cc9d-e6bf-4e47-a7c0-e02c8778b8ec" height="70px" />  guana </h2>

`guana` is the `Grpc User AgeNt Auth`.

## Building

This project can be built using a [Nix shell](https://nixos.wiki/wiki/Development_environment_with_nix-shell), which provides a consistent and reproducible development by managing dependencies.

Once Nix is installed, run `nix-shell` in the root folder to spawn a subshell with the pre-configured development environment.

### Setting up Rust

[Rustup](https://rustup.rs/), the Rust toolchain installer, is supplied by the Nix shell.

Check if a working Rust toolchain is installed:

```bash
rustc --version # rustc 1.70.0 (90c541806 2023-05-31)
cargo --version # cargo 1.70.0 (ec8a8a0ca 2023-04-25)
```

If for any reason one of the two commands above is not found, ensure the stable toolchain is installed:

```bash
rustup toolchain install stable
```

### Building 

## Project structure

* `proto`
    - The Protobuf schema for the project.
* `guana-grpc-types`
    - Shared crate for the client and server defined in the [Protobuf schema](https://todo). This is the crate the compiles the `.proto` file and generates the [`prost`]() code to interface with it.
* `guana-grpc-server`
    - ss