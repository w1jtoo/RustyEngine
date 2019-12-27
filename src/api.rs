extern crate glium;

use glium::glutin::EventsLoop;
use glium::{Display, Surface};
use std::cell::Cell;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Vertex {
        Vertex { position: [x, y] }
    }
}

pub struct ObjectRederer<'a> {
    shape: &'a Vec<Vertex>,
    vertex_shader_src: &'a str,
    fragment_shader_src: &'a str,
    geometry_shader: Option<&'a str>,
}

impl<'a> ObjectRederer<'a> {
    pub fn new(
        shape: &'a Vec<Vertex>,
        vertex_shader_src: &'a str,
        fragment_shader_src: &'a str,
        geometry_shader: Option<&'a str>,
    ) -> ObjectRederer<'a> {
        ObjectRederer {
            shape: &shape,
            vertex_shader_src: vertex_shader_src,
            fragment_shader_src: fragment_shader_src,
            geometry_shader: None,
        }
    }
}

pub struct Application<'a> {
    pub closed: bool,
    pub events_loop: &'a mut EventsLoop,
    pub display: Display,
    pub render_objects: Vec<ObjectRederer<'a>>,
}

impl<'a> Application<'a> {
    pub fn run(&mut self) {
        while !self.closed {
            let mut target = self.display.draw();
            target.clear_color(0.0, 0.0, 1.0, 1.0);
            implement_vertex!(Vertex, position);

            for obj in &self.render_objects {
                let vertex_buffer = glium::VertexBuffer::new(&self.display, obj.shape).unwrap();
                let program = glium::Program::from_source(
                    &self.display,
                    obj.vertex_shader_src,
                    obj.fragment_shader_src,
                    obj.geometry_shader,
                )
                .unwrap();
                target
                    .draw(
                        &vertex_buffer,
                        glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                        &program,
                        &glium::uniforms::EmptyUniforms,
                        &Default::default(),
                    )
                    .unwrap();
            }

            target.finish().unwrap();
            let mut closed = false;
            self.events_loop.poll_events(|ev| match ev {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    glium::glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            });

            self.closed = closed;
        }
    }
}

impl<'a> Application<'a> {
    pub fn new(self) -> Self {
        todo!();
        // let wb = WindowBuilder::new();
        // let cb = ContextBuilder::new();
        // let mut events_loop = EventsLoop::new();
        // Application {
        //     events_loop: &events_loop,
        //     display: Display::new(wb, cb, &events_loop).unwrap(), // <- TODO: Add logging system to
        //     closed: false,
        //     renderObjects: Vec::new(),
        // }
    }

    pub fn close(&mut self) {
        self.closed = true;
    }

    pub fn add_object(&mut self, object_rederer: ObjectRederer<'a>) {
        self.render_objects.push(object_rederer);
    }
}
