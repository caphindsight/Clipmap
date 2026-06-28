# <img src="https://raw.githubusercontent.com/caphindsight/Clipmap/refs/heads/master/addons/clipmap/clipmap_mesh.svg" width="24"> ClipmapMesh

Adds a mesh primitive called `ClipmapMesh`.

`ClipmapMesh` looks like a plane mesh, but contains regions of variable detalization.
The sizes and detalizations of the regions are configurable through inspector properties.
The regions are stitched together seamlessly.

Useful for large surfaces with variable detalization, like terrain, ocean, etc.
Implemented via gdextension. The repo contains optimized builds for Windows and Linux. Builds are reproducible from the code in the repo.

## Installation

### The easy way

Just copy `addons/clipmap` into your Godot project.

### The hard way

This folder contains executable files (`.dll` for Windows, `.so` for Linux-based, `.dylib` for MacOS / derivatives).
If you don't trust the executables, you can build your own and/or verify existing ones with `cargo`:

```
$ cd addons/clipmap/rust
$ cargo build --target=<YOUR_TARGET> [--release]
```

Here `YOUR_TARGET` should be one of:

*   `x86_64-pc-windows-gnu`
*   `x86_64-unknown-linux-gnu`
*   `x86_64-apple-darwin`
*   `aarch64-unknown-linux-gnu`
*   `aarch64-apple-darwin`

You can also build for different architectures this way.

## Preview:

![preview](https://raw.githubusercontent.com/caphindsight/Clipmap/refs/heads/master/thumbnail.png)
