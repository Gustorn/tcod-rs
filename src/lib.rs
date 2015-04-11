#[macro_use] extern crate bitflags;

pub use bindings::{AsNative, FromNative};
pub use colors::Color;
pub use console::{Console, BackgroundFlag, Renderer, FontFlags, TextAlignment};
pub use map::Map;

pub mod chars;
pub mod colors;
pub mod console;
pub mod input;
pub mod map;
pub mod pathfinding;
pub mod system;

mod bindings;


pub type RootConsole = console::Root; 
pub type OffscreenConsole = console::Offscreen;

