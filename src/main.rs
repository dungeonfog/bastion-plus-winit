use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use bastion::prelude::*;

mod renderer;
use renderer::{RenderMessage, Renderer};

#[derive(Debug)]
struct QuitMessage;

fn main() {
    Bastion::init();
    Bastion::start();

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut children_ref = None;

    let supervisor = Bastion::supervisor(|sp| {
        let sp = sp.with_strategy(SupervisionStrategy::OneForOne);
        children_ref = Some(sp.children_ref(|children| children.with_exec(Renderer::exec)));
        sp
    })
    .expect("Couldn't create the supervisor.");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                log::info!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { .. },
                ..
            } => {
                if let Some(children_ref) = &children_ref {
                    let render_ref = &children_ref.elems()[0];
                    render_ref
                        .tell_anonymously(RenderMessage::Foo)
                        .expect("Failed sending message");
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {}
            Event::LoopDestroyed => {
                supervisor
                    .broadcast(QuitMessage)
                    .expect("Failed sending QuitMessage");

                Bastion::stop();
                Bastion::block_until_stopped();
            }
            _ => (),
        }
    });
}
