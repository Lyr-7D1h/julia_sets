use glfw::{self, Context, Key, Window, WindowEvent, WindowMode};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = glfw
        .create_window(500, 500, "Julia Sets", WindowMode::Windowed)
        .expect("Failed to make window");

    window.set_key_polling(true);
    window.make_current();

    while !window.should_close() {
        window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{event:?}");
            handle_window_event(event, &mut window);
        }
    }
}

fn handle_window_event(event: WindowEvent, window: &mut Window) {
    match event {
        glfw::WindowEvent::Close => {
            window.set_should_close(true);
        }
        glfw::WindowEvent::Key(Key::Escape, _, glfw::Action::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}
