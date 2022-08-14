use std::ffi::CStr;
use glutin::{self, event::{WindowEvent,  VirtualKeyCode, Event}, window::{ WindowBuilder, Window}, event_loop::ControlFlow, PossiblyCurrent, ContextWrapper, platform::unix::WindowBuilderExtUnix};
use gl33::{global_loader::{glClearColor, load_global_gl, glClear}, GL_COLOR_BUFFER_BIT};

fn main() {
    let el = glutin::event_loop::EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("dev_julia_sets")
        .with_name("dev_julia_sets","dev_julia_sets")
        .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
    let context = glutin::ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(wb, &el)
        .unwrap();
    
    let mut context = unsafe {
        context.make_current().unwrap()
    };


    unsafe {
        load_global_gl(&|procname| {
            let procname = CStr::from_ptr(procname as *const i8).to_str().unwrap();
            context.get_proc_address(procname)
        });
        glClearColor(0.1, 0.2, 0.3, 1.0)
    }

    el.run(move |event, _, control_flow| {
        unsafe {
            glClear(GL_COLOR_BUFFER_BIT);
        }
        handle_window_event(event, control_flow, &mut context);
        context.swap_buffers().unwrap();
    })
}

fn handle_window_event(event: Event<()>, control_flow: &mut ControlFlow, context: &mut ContextWrapper<PossiblyCurrent, Window>) {
    match event {
        Event::WindowEvent {event, ..} => {
            match event {
                WindowEvent::CloseRequested => {
                                control_flow.set_exit();
                }
                WindowEvent::KeyboardInput {input, ..} => {
                    if let Some(k) = input.virtual_keycode {
                        match k {
                            VirtualKeyCode::Escape => {
                                println!("Exiting program");
                                control_flow.set_exit();
                            }
                            _ => ()
                        }
                    }
                }
                _ => ()
            }
        }
        _ => ()
    }
}
