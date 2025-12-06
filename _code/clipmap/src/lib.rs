use godot::prelude::*;

struct ClipmapExtLib;

#[gdextension]
unsafe impl ExtensionLibrary for ClipmapExtLib {}

mod clipmap;
