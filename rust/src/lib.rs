use godot::prelude::*;

mod maze;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
