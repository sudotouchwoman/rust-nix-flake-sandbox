# Sandbox project for Rust with Nix Flakes

This repository contains a dummy rust project setup with Nix Flake.
Flakes act as reproducible source of truth for `nix`, and it can be very convenient to
have a completely reproducible development environment (as much as `nix` can get).

### Useful links

+ https://dev.to/johnreillymurray/rust-environment-and-docker-build-with-nix-flakes-19c1
+ https://discourse.nixos.org/t/rust-local-documentation/32953
+ https://ayats.org/blog/nix-rustup
+ https://github.com/nix-community/fenix

### How to use

You will need `nix` and `flakes` support enabled. Then just `nix develop` will do.
You can also use it with `direnv` (see `.envrc`), e.g. with VSCode extension.

You can build local copy for rust toolchain with `fenix` like this:

```shell
nix build fenix#latest.rust-docs
firefox result/share/doc/rust/html/index.html

# or alternatively
xdg-open result/share/doc/rust/html/index.html

# or with browser-specific commands
google-chrome --incognito result/share/doc/rust/html/index.html
```

It feels very unnatural for some reason, but you can actually run `rust`-built binaries
under `valgrind`, e.g. like this with `nix`:

```shell
nix run nixpkgs#valgrind ./target/debug/main
```
