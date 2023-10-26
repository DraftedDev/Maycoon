use std::num::NonZeroU32;

use skia_safe::Color4f;
use softbuffer::{Context, Surface};
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoopBuilder;
use winit::platform::windows::EventLoopBuilderExtWindows;
use winit::window::{Fullscreen, WindowBuilder};

use crate::app::config::AppConfig;
use crate::app::page::Page;
use crate::canvas::Canvas;

pub mod config;
pub mod page;
pub mod widget;

pub struct MayApp {
    pub config: AppConfig,
}

impl MayApp {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub fn run<S>(self, mut ui: Page<S>) {
        let event_loop = EventLoopBuilder::new()
            .with_any_thread(self.config.any_thread)
            .build();

        // set up window in closure to better customize it without wasting memory afterwards
        let window = {
            let config = &self.config.window;

            let fullscreen = if config.fullscreen {
                Some(Fullscreen::Borderless(event_loop.primary_monitor()))
            } else {
                None
            };

            WindowBuilder::new()
                .with_decorations(config.decorations)
                .with_fullscreen(fullscreen)
                .with_maximized(config.maximized)
                .with_inner_size(config.size)
                .with_max_inner_size(config.max_size)
                .with_min_inner_size(config.min_size)
                .with_position(config.position)
                .with_transparent(config.transparent)
                .with_window_level(config.level)
                .with_visible(config.visible)
                .with_title(&config.title)
                .build(&event_loop)
                .expect("Window: Failed to create")
        };

        let context =
            unsafe { Context::new(&window) }.expect("Softbuffer: Failed to create buffer context");

        let mut surface = unsafe { Surface::new(&context, &window) }
            .expect("Softbuffer: Failed to create window surface");

        // global window size
        let (mut width, mut height) = {
            let size = window.inner_size();
            (size.width, size.height)
        };

        let mut canvas = Canvas::new(width as i32, height as i32);

        // run the app and window event loop
        event_loop.run(move |event, _, control_flow| {
            control_flow.set_wait();

            match event {
                Event::RedrawRequested(window_id) if window_id == window.id() => {
                    // check if state changed, to avoid rendering without changes
                    if ui.state_changed {
                        let mut buffer = surface
                            .buffer_mut()
                            .expect("Softbuffer: Failed to get buffer");

                        // apply background color first
                        canvas
                            .skia_canvas()
                            .clear(Color4f::from(self.config.clear_color));

                        canvas.draw_root(&mut ui.widgets, width as f32, height as f32);

                        // apply canvas to buffer
                        {
                            let mut data = canvas.data();
                            buffer.iter_mut().enumerate().for_each(|(idx, px)| {
                                *px = *data.get_mut(idx).unwrap_or_else(|| {
                                    panic!("Softbuffer: Failed to get pixel: {}", idx);
                                });
                            });
                        }

                        // update page state and present the drawn buffer
                        ui.state_changed = false;
                        buffer
                            .present()
                            .expect("Softbuffer: Failed to present buffer");
                    }
                }

                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::Resized(size),
                } if window_id == window.id() => {
                    // don't resize if window is minimized, since some OS's (like windows) resize the window to 0x0 when minimized
                    if !window.is_minimized().unwrap_or(false) {
                        // update sizes
                        width = size.width;
                        height = size.height;

                        canvas.resize(width as i32, height as i32);

                        surface
                            .resize(
                                NonZeroU32::new(width).unwrap(),
                                NonZeroU32::new(height).unwrap(),
                            )
                            .expect("Softbuffer: Failed to resize window surface");

                        // change state, so that we can redraw widgets to fit the sizes
                        ui.state_changed = true;
                    }
                }

                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => {
                    control_flow.set_exit();
                }

                _ => {}
            }
        });
    }
}
