pub use skia_safe;

mod backend;
pub use backend::Backend;
pub mod window;
pub mod settings;
pub use settings::Settings;

pub use iced_graphics::{Error, Viewport};
pub use iced_native::Theme;

pub use iced_native::alignment;
pub use iced_native::{Alignment, Background, Color, Command, Length, Vector};

pub type Renderer<Theme = iced_native::Theme> = iced_graphics::Renderer<Backend, Theme>;