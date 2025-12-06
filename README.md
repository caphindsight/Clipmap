# Clipmap

Available in Godot's [asset library](https://godotengine.org/asset-library/asset/4545).

Adds a mesh primitive called `ClipmapMesh`.

`ClipmapMesh` looks like a plane mesh, but contains regions of variable detalization.
The sizes and detalizations of the regions are configurable through inspector properties.
The regions are stitched together seamlessly.

Useful for large surfaces with variable detalization, like terrain, ocean, etc.
Implemented via gdextension. The repo contains optimized builds for Windows and Linux. Builds are reproducible from the code in the repo.

## Building from sources

1. Install `rustup` and run `rustup target add <your_arch>`.
2. `cd _code && make rel`.
3. Copy-paste the `addons/clipmap` dir into your Godot project.

## Preview:

![preview](https://raw.githubusercontent.com/caphindsight/Clipmap/refs/heads/master/_code/preview.png)

P.S. if anyone can make a better `ClipmapMesh` icon, I will be eternally thankful :)
