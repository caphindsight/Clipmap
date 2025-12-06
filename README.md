# Clipmap

**No longer available in Godot's asset library.**
For reasons behind this, see later.

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

## Why not upload to the Godot Asset Library?

I tried uploading it there, and had a bad experience.

The addon got accepted on the first try, but a follow-up bug fix edit was rejected for an unrelated reason.

The moderators there were unhappy with the MacOS build missing from the repo.
There are personal reasons why I do not wish to include the MacOS build in the repo.
It is trivial to build for MacOS, and instructions are included above.

This leaves the addon hanging in a "bad state": it was accepted initially and is available for download,
but I am unable to make changes, because they get rejected for an unrealated reason that is unfortunately a deal breaker for me.
There are currently no appeal mechanisms in the Asset Library.
This puts me in an unnecessarily difficult situation, so I decided that I would rather maintain a fresh version here, than leave a stale version that I cannot update.

Unfortunately, I also cannot delete an addon from the Asset Library.

I do enjoy using the Godot Asset Library, and I wish them all the best in their future growth.
I just hope they revisit the current policies, and add a mechanism for rejection appeals.
I also hope that they add a compatibility feature (e.g. I would mark this addon as only compatible with Windows and Linux builds).

## Preview:

![preview](https://raw.githubusercontent.com/caphindsight/Clipmap/refs/heads/master/_code/preview.png)

P.S. if anyone can make a better `ClipmapMesh` icon, I will be eternally thankful :)
