#[macro_use]
extern crate glium;

pub mod api;

use api::{Application, ObjectRederer, Vertex};

fn main() {
    let mut events_loop = glium::glutin::EventsLoop::new();
    let wb = glium::glutin::WindowBuilder::new();
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let mut app = Application {
        closed: false,
        events_loop: &mut events_loop,
        display: display,
        render_objects: Vec::new(),
    };

    let vertex1 = Vertex::new(1.0, 1.0);
    let vertex2 = Vertex::new(0.0, 1.0);
    let vertex3 = Vertex::new(1.0, 0.0);
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_shader = r#"
    #version 140
    in vec2 position;
    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;

    let fragment_shader = r#"
    #version 140
    out vec4 color;
    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;

    let obj = ObjectRederer::new(&shape, vertex_shader, fragment_shader, None);

    app.add_object(obj);
    app.run();
}
