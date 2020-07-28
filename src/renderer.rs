use bastion::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderMessage {
    Foo,
    Bar,
}

pub struct Renderer {}

impl Renderer {
    pub async fn exec(ctx: BastionContext) -> Result<(), ()> {
        loop {
            msg! { ctx.recv().await?,
                msg: RenderMessage => {
                    log::debug!("render msg: {:?}", msg);
                };
                ref msg: super::QuitMessage => {
                    log::info!("Renderer shutting down.");
                    break;
                };
                _: _ => panic!("Unknown message received");
            }
        }

        Ok(())
    }
    // fn new() -> Self {
    //     Self {}
    // }
}
