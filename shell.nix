with import <nixpkgs> {};

stdenv.mkDerivation {
    name = "guana";

    buildInputs = [
        openssl.dev
        openssl
        protobuf
        rustup
    ];

    OPENSSL_DEV=openssl.dev;
}