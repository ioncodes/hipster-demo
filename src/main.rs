#[macro_use]
extern crate glium;

use std::collections::HashMap;

fn main() {
    let mut scenes = HashMap::new();
    scenes.insert(0, Scene {
        add_size: 0.001,
        add_time: 0.0,
        fragment: include_str!("../shaders/circle.frag").to_owned(),
        name: "Circle".to_owned()
    });
    scenes.insert(1, Scene {
        add_size: 0.0,
        add_time: 0.01,
        fragment: include_str!("../shaders/color_build.frag").to_owned(),
        name: "Color Build".to_owned()
    });
    scenes.insert(2, Scene {
        add_size: 0.0,
        add_time: 0.01,
        fragment: include_str!("../shaders/color_loop.frag").to_owned(),
        name: "Color Loop".to_owned()
    });
    scenes.insert(3, Scene {
        add_size: 0.0,
        add_time: 0.01,
        fragment: include_str!("../shaders/color_loop_metal.frag").to_owned(),
        name: "Color Loop Metal".to_owned()
    });
    scenes.insert(4, Scene {
        add_size: 0.0,
        add_time: 0.01,
        fragment: include_str!("../shaders/color_unfold.frag").to_owned(),
        name: "Color Unfold".to_owned()
    });
    scenes.insert(5, Scene {
        add_size: 0.0,
        add_time: 0.01,
        fragment: include_str!("../shaders/tunnel.frag").to_owned(),
        name: "Tunnel".to_owned()
    });
    scenes.insert(6, Scene {
        add_size: 0.0,
        add_time: 0.01,
        fragment: include_str!("../shaders/underworld.frag").to_owned(),
        name: "Underworld".to_owned()
    });
    scenes.insert(7, Scene {
        add_size: 0.0,
        add_time: 0.01,
        fragment: include_str!("../shaders/chip.frag").to_owned(),
        name: "Chip".to_owned()
    });
    scenes.insert(8, Scene {
        add_size: 0.0,
        add_time: 0.01,
        fragment: include_str!("../shaders/underworld_ball.frag").to_owned(),
        name: "Underworld 2".to_owned()
    });
    scenes.insert(9, Scene {
        add_size: 0.0,
        add_time: 0.01,
        fragment: include_str!("../shaders/ball_reflector.frag").to_owned(),
        name: "Ball Reflector".to_owned()
    });
    scenes.insert(10, Scene {
        add_size: 0.0,
        add_time: 0.1,
        fragment: include_str!("../shaders/strange_room.frag").to_owned(),
        name: "Strange Room".to_owned()
    });
    scenes.insert(11, Scene {
        add_size: 0.0,
        add_time: 0.01,
        fragment: include_str!("../shaders/3d_mover.frag").to_owned(),
        name: "Staging".to_owned()
    });
    scenes.insert(12, Scene {
        add_size: 0.0,
        add_time: 0.01,
        fragment: include_str!("../shaders/disco.frag").to_owned(),
        name: "Disco".to_owned()
    });
    scenes.insert(13, Scene {
        add_size: 0.0,
        add_time: 0.04,
        fragment: include_str!("../shaders/brainfuck.frag").to_owned(),
        name: "Brainfuck".to_owned()
    });

    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let mut scene_id = 0;
    let mut scene = scenes.get(&scene_id).unwrap();
    let window = glutin::WindowBuilder::new()
                        .with_dimensions(1024, 768)
                        .with_title("hipster demo");
    let context = glutin::ContextBuilder::new()
                        .with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let shape = glium::vertex::VertexBuffer::new(&display, &[
        Vertex { position: [-1.0,  1.0] },
        Vertex { position: [ 1.0,  1.0] },
        Vertex { position: [-1.0, -1.0] },
        Vertex { position: [ 1.0, -1.0] },
    ]).unwrap();

    let vertex_shader_src = include_str!("../shaders/background.vert").to_owned();

    let mut fragment_shader_src = &scene.fragment;

    let mut program =
        glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None)
            .unwrap();

    let mut closed = false;
    let mut size: f32 = 0.0;
    let mut time: f32 = 0.0;
    let mut add_size = scene.add_size;
    let mut add_time = scene.add_time;

    while !closed {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        let uniforms =
            uniform! { 
                resolution: [1024.0 as f32, 768.0 as f32],
                size: size.sin(), // only needed for circle
                time: time
            };

        size += add_size;
        time += add_time;

        target
            .draw(
                &shape,
                &glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        // todo: make own loader
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glutin::WindowEvent::Closed => closed = true,
                    glutin::WindowEvent::KeyboardInput { input, .. } if glutin::ElementState::Pressed == input.state  => {
                    if let Some(glutin::VirtualKeyCode::Left) = input.virtual_keycode {
                        if scene_id == 0 {
                            scene_id = scenes.len() - 1;
                        } else {
                            scene_id -= 1;
                        }
                        scene = scenes.get(&scene_id).unwrap();
                        add_size = scene.add_size;
                        add_time = scene.add_time;
                        fragment_shader_src = &scene.fragment;
                        size = 0.0;
                        time = 0.0;
                        program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();
                        println!("Rendering: {:?}", &scene.name);
                    } else if let Some(glutin::VirtualKeyCode::Right) = input.virtual_keycode {
                        if scene_id == scenes.len() - 1 {
                            scene_id = 0;
                        } else {
                            scene_id += 1;
                        }
                        scene = scenes.get(&scene_id).unwrap();
                        add_size = scene.add_size;
                        add_time = scene.add_time;
                        fragment_shader_src = &scene.fragment;
                        size = 0.0;
                        time = 0.0;
                        program = glium::Program::from_source(&display, &vertex_shader_src, &fragment_shader_src, None).unwrap();
                        println!("Rendering: {:?}", &scene.name);
                    }
                },
                    _ => (),
                }
            },
            _ => (),
        });
    }
}

struct Scene {
    pub add_size: f32,
    pub add_time: f32,
    pub fragment: String,
    pub name: String
}