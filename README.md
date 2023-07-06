<h2 align="center"><img align="center" src="https://github.com/vrmiguel/vrmiguel/assets/36349314/5170cc9d-e6bf-4e47-a7c0-e02c8778b8ec" height="70px" />  guana </h2>

`guana` is the `Grpc User AgeNt Auth`.

For now, it works as follows:

* Given a Mozilla Firefox User-Agent, the server returns `ALLOW`
* Given a Safari User-Agent, the server returns `BLOCK`
* Given something else, the server returns `UNKNOWN`

While the functionality is simplistic, Guana serves as a solid foundation to build upon and develop more complex gRPC-based projects.

For an example of it's usage, check out [`guana-client`](guana-client).

## Project structure

In order to be modular, this project is structured as follows:

* [`proto`](proto)
    - The Protobuf schema for the project.
* [`guana-grpc-types`](guana-grpc-types)
    - Shared crate for the client and server defined in the [Protobuf schema](https://todo). This is the crate the compiles the `.proto` file and generates the [`prost`](https://crates.io/crates/prost) code to interface with it.
* [`guana-service`](guana-service)
    - The library that implements the `UserAgentAnalyzer` gRPC service.
    - This is useful if you'd like to embed Guana in another Rust project, e.g. as an additional service within a larger gRPC server. See its README for more information.
* [`guana-server`](guana-server)
    - The binary that runs the highly performant Guana gRPC server. This is useful if you want to run Guana standalone.
* [`guana-client`](guana-client)
    - Client binary and library for the Guana gRPC server. See it's README for more information.

## Build instructions

### With Nix

This project can be built using a [Nix shell](https://nixos.wiki/wiki/Development_environment_with_nix-shell), which provides a consistent and reproducible development by managing dependencies.

Once Nix is installed, run `nix-shell` in the root folder to spawn a subshell with the pre-configured development environment.

```bash
$ nix-shell
$ # You're now in a Nix subshell with the supplied dependencies
$ protoc --version # libprotoc 3.21.12
$ rustc --version  # rustc 1.7.0
```

### Without Nix

* On Debian-based systems:
    - `sudo apt install -y protobuf-compiler libprotobuf-dev`
    - And [Rustup](https://rustup.rs).
* On Arch-based systems:
    - `sudo pacman -S protobuf rustup`
* On macOS with Homebrew:
    - `brew install protobuf rustup`

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

### Building

Once the dependencies are met, build the parts of the project you'd like.
For instance, the process for running Guana locally is:

```bash
# Session 1
$ nix-shell # Enter the nix-shell
$ cd guana-server/
$ cargo run --release
2023-07-05T19:50:05.308188Z  INFO guana_server: Server address not set. Using a default value
2023-07-05T19:50:05.308596Z  INFO guana_server: Starting server on [::1]:50051
```

In another session or terminal, run:

```bash
# Session 2
$ nix-shell 
$ cd guana-client/
$ cargo run --release
> Warn Address not set. Connecting to '[::1]:50051'
>> Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/114.0
> User-Agent Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/114.0: ALLOW
```