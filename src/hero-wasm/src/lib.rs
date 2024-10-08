use std::mem::transmute;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use log::error;
use state::State;
use wasm_bindgen::prelude::*;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::platform::web::{EventLoopExtWebSys, WindowExtWebSys};
use winit::window::WindowBuilder;

mod sim;
mod state;

static BODY_1_MASS: AtomicU32 = AtomicU32::new(unsafe { transmute(10.0f32) });
static BODY_2_MASS: AtomicU32 = AtomicU32::new(unsafe { transmute(10.0f32) });
static BODY_3_MASS: AtomicU32 = AtomicU32::new(unsafe { transmute(10.0f32) });
static TIME_SCALE: AtomicU32 = AtomicU32::new(unsafe { transmute(1.0f32) });
static PAUSED: AtomicBool = AtomicBool::new(true);
static RESET_REQUESTED: AtomicBool = AtomicBool::new(true);

#[wasm_bindgen]
pub fn set_body_1_mass(new_mass: f32) {
    BODY_1_MASS.store(new_mass.to_bits(), Ordering::Relaxed);
}

#[wasm_bindgen]
pub fn set_body_2_mass(new_mass: f32) {
    BODY_2_MASS.store(new_mass.to_bits(), Ordering::Relaxed);
}

#[wasm_bindgen]
pub fn set_body_3_mass(new_mass: f32) {
    BODY_3_MASS.store(new_mass.to_bits(), Ordering::Relaxed);
}

#[wasm_bindgen]
pub fn pause() {
    PAUSED.store(true, Ordering::Relaxed);
}

#[wasm_bindgen]
pub fn play() {
    PAUSED.store(false, Ordering::Relaxed);
}

#[wasm_bindgen]
pub fn reset() {
    RESET_REQUESTED.store(true, Ordering::Relaxed);
}

#[wasm_bindgen]
pub fn set_time_scale(new_scale: f32) {
    TIME_SCALE.store(new_scale.to_bits(), Ordering::Relaxed);
}

#[wasm_bindgen(start)]
pub async fn run() {
    // Redirect panics to the console (debugging)
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let window = Box::leak(Box::new(window));
    let html_window = web_sys::window().expect("Couldn't get window");
    let scale = html_window.device_pixel_ratio();

    let container = html_window
        .document()
        .and_then(|doc| {
            let dst = doc.get_element_by_id("hero-wasm-container")?;
            let canvas = web_sys::Element::from(window.canvas()?);
            dst.append_child(&canvas).ok()?;
            Some(dst)
        })
        .expect("Couldn't append canvas to document body.");

    let mut state = State::new(&*window).await;

    event_loop.spawn(move |event, _control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::RedrawRequested => {
                            let rect = container.get_bounding_client_rect();
                            state.match_parent(PhysicalSize::new(
                                (rect.width() * scale).round() as u32,
                                (rect.height() * scale).round() as u32,
                            ));
                            state.update();
                            match state.render() {
                                Ok(_) => {}
                                // Reconfigure the surface if lost
                                Err(wgpu::SurfaceError::Lost) => state.resize(state.get_size()),
                                // All other errors (Outdated, Timeout) should be resolved by the next frame
                                Err(e) => error!("{:?}", e),
                            }
                        }
                        _ => {}
                    }
                }
            }

            Event::AboutToWait => {
                // RedrawRequested will only trigger once unless we manually
                // request it.
                state.window().request_redraw();
            }
            _ => {}
        }
    });
}
