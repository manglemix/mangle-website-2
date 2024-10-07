use log::{error, info};
use state::State;
use wasm_bindgen::prelude::*;
use web_sys::console::info;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::platform::web::WindowExtWebSys;
use winit::window::WindowBuilder;

mod state;

// // export Rust function greet to be used in JS/TS, the same function signature will be used in JS/TS
// #[wasm_bindgen]
// pub fn greet(str: &str) {
//     alert(&format!("Hello, {}!", str));
// }

#[wasm_bindgen(start)]
pub async fn run() {
    // Redirect panics to the console (debugging)
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let _ = window.request_inner_size(PhysicalSize::new(450, 400));

    let container = web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| {
            let dst = doc.get_element_by_id("hero-wasm-container")?;
            let canvas = web_sys::Element::from(window.canvas()?);
            dst.append_child(&canvas).ok()?;
            Some(dst)
        })
        .expect("Couldn't append canvas to document body.");

    let mut state = State::new(&window).await;
    info!("State created");

    event_loop
        .run(move |event, _control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == state.window().id() => {
                    if !state.input(event) {
                        match event {
                            // WindowEvent::Resized(physical_size) => {
                            //     state.resize(*physical_size);
                            // }
                            WindowEvent::RedrawRequested => {
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
        })
        .expect("Event loop failed");
}
