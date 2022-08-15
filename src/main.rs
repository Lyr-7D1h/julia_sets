use gl33::{
    global_loader::{
        glAttachShader, glBindBuffer, glBindVertexArray, glBufferData, glClear, glClearColor,
        glCompileShader, glCreateProgram, glCreateShader, glDeleteShader, glDrawArrays,
        glEnableVertexAttribArray, glGenBuffers, glGenVertexArrays, glGetProgramInfoLog,
        glGetProgramiv, glGetShaderInfoLog, glGetShaderiv, glLinkProgram, glShaderSource,
        glUseProgram, glVertexAttribPointer, load_global_gl,
    },
    GL_ARRAY_BUFFER, GL_COLOR_BUFFER_BIT, GL_COMPILE_STATUS, GL_FLOAT, GL_FRAGMENT_SHADER,
    GL_LINK_STATUS, GL_STATIC_DRAW, GL_TRIANGLES, GL_VERTEX_SHADER,
};
use glutin::{
    self,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    platform::unix::WindowBuilderExtUnix,
    window::{Window, WindowBuilder},
    Api, ContextWrapper, GlProfile, PossiblyCurrent,
};
use std::{
    ffi::CStr,
    mem::{size_of, size_of_val},
};

type Vertex = [f32; 3];

fn main() {
    let el = glutin::event_loop::EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("dev_julia_sets")
        .with_name("dev_julia_sets", "dev_julia_sets")
        .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
    let context = glutin::ContextBuilder::new()
        .with_vsync(true) // Make buffer swapping blocking until rendering has arrived on screen
        .with_gl_profile(GlProfile::Core) // use version 3.3
        .with_gl(glutin::GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_windowed(wb, &el)
        .unwrap();

    let mut context = unsafe { context.make_current().unwrap() };

    unsafe {
        load_global_gl(&|procname| {
            let procname = CStr::from_ptr(procname as *const i8).to_str().unwrap();
            context.get_proc_address(procname)
        });
        glClearColor(0.1, 0.2, 0.3, 1.0);
    }

    unsafe {
        let mut vao = 0;
        glGenVertexArrays(1, &mut vao); // Generate vertex array object name
        assert_ne!(vao, 0);
        glBindVertexArray(vao); // Create and bind vertex array object
    }

    unsafe {
        let mut vbo = 0;
        glGenBuffers(1, &mut vbo); // Generate unused buffer object name (u32)
        assert_ne!(vbo, 0); // Name has to be non-zero
        glBindBuffer(GL_ARRAY_BUFFER, vbo); // Bind buffer object to GL_ARRAY_BUFFER
    }

    unsafe {
        const VERTICES: [Vertex; 3] = [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];
        glBufferData(
            // specify data format
            GL_ARRAY_BUFFER,
            size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            GL_STATIC_DRAW,
        )
    }

    unsafe {
        // describe how to gpu will interpret buffer bytes
        glVertexAttribPointer(
            0, // Index: used in glsl shader
            3,
            GL_FLOAT,
            0, // False
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        glEnableVertexAttribArray(0);
    }

    let vertex_shader = load_vertex_shader();
    let fragment_shader = load_fragment_shader();

    let shader_program = glCreateProgram();
    glAttachShader(shader_program, vertex_shader);
    glAttachShader(shader_program, fragment_shader);
    glLinkProgram(shader_program);

    unsafe {
        let mut success = 0;
        glGetProgramiv(shader_program, GL_LINK_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            glGetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }
    }

    // Mark shaders to be removed
    // will only be removed when shader gets unattached
    glDeleteShader(vertex_shader);
    glDeleteShader(fragment_shader);

    // Use shader!
    glUseProgram(shader_program);

    el.run(move |event, _, control_flow| {
        unsafe {
            glClear(GL_COLOR_BUFFER_BIT);
            glDrawArrays(GL_TRIANGLES, 0, 3);
        }
        handle_window_event(event, control_flow, &mut context);
        context.swap_buffers().unwrap();
    })
}

fn load_vertex_shader() -> u32 {
    let vertex_shader = glCreateShader(GL_VERTEX_SHADER);
    assert_ne!(vertex_shader, 0);
    // shader written in GLSL
    const VERT_SHADER: &str = r#"#version 330 core
        layout (location = 0) in vec3 pos;
        void main() {
            gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
        }
    "#;

    unsafe {
        glShaderSource(
            // Load a glsl string into memory
            vertex_shader,
            1,
            &(VERT_SHADER.as_ptr().cast()),
            &(VERT_SHADER.len().try_into().unwrap()),
        );
        glCompileShader(vertex_shader);
    }

    // Validate it got compiled successfully
    unsafe {
        let mut success = 0;
        // get shader int vector
        glGetShaderiv(vertex_shader, GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0;
            glGetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
        }
    }

    return vertex_shader;
}

// TODO: make generic shader function
fn load_fragment_shader() -> u32 {
    let fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
    assert_ne!(fragment_shader, 0);
    const FRAG_SHADER: &str = r#"#version 330 core
        out vec4 final_color;
      
        void main() {
            final_color = vec4(1.0, 0.5, 0.2, 1.0);
        }
    "#;

    unsafe {
        glShaderSource(
            fragment_shader,
            1,
            &(FRAG_SHADER.as_ptr().cast()),
            &(FRAG_SHADER.len().try_into().unwrap()),
        );
        glCompileShader(fragment_shader);
    }

    // Validate it got compiled successfully
    unsafe {
        let mut success = 0;
        // get shader int vector
        glGetShaderiv(fragment_shader, GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0;
            glGetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
        }
    }

    return fragment_shader;
}

fn handle_window_event(
    event: Event<()>,
    control_flow: &mut ControlFlow,
    context: &mut ContextWrapper<PossiblyCurrent, Window>,
) {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::CloseRequested => {
                control_flow.set_exit();
            }
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(k) = input.virtual_keycode {
                    match k {
                        VirtualKeyCode::Escape => {
                            println!("Exiting program");
                            control_flow.set_exit();
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        },
        _ => (),
    }
}
