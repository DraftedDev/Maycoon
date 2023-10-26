use skia_safe::Color;
use winit::dpi::{LogicalPosition, LogicalSize, Position, Size};
use winit::window::WindowLevel;

pub struct AppConfig {
    pub window: WindowConfig,
    pub any_thread: bool,
    pub clear_color: Color,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window: WindowConfig::default(),
            any_thread: false,
            clear_color: Color::WHITE,
        }
    }
}

pub struct WindowConfig {
    pub title: String,
    pub level: WindowLevel,

    pub transparent: bool,
    pub fullscreen: bool,
    pub resizable: bool,
    pub decorations: bool,
    pub maximized: bool,
    pub visible: bool,

    pub position: Position,
    pub min_size: Size,
    pub size: Size,
    pub max_size: Size,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "New Maycoon App".to_string(),
            level: WindowLevel::Normal,
            transparent: false,
            fullscreen: false,
            resizable: true,
            decorations: true,
            maximized: false,
            visible: true,
            position: Position::from(LogicalPosition::new(0, 0)),
            min_size: Size::from(LogicalSize::new(10, 10)),
            size: Size::from(LogicalSize::new(1000, 600)),
            max_size: Size::from(LogicalSize::new(10000, 10000)),
        }
    }
}
