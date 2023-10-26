use may_core::app::config::{AppConfig, WindowConfig};
use may_core::app::page::Page;
use may_core::app::MayApp;
use may_core::widgets::text::Text;

struct AppState();

fn main() {
    MayApp::new(AppConfig {
        window: WindowConfig {
            title: "Hello World!".to_string(),
            ..Default::default()
        },
        ..Default::default()
    })
    .run(Page::build(vec![Text::new("Hello World!")], AppState()));
}
