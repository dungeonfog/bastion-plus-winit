use bastion::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderMessage {
    Foo,
    Bar,
}

pub struct Renderer {}

impl Renderer {
    pub async fn exec(ctx: BastionContext) -> Result<(), ()> {
        log::debug!("Starting renderer...");
        let task = blocking! {
            log::debug!("On blocking thread");
            run!(async move {
                log::debug!("Running blocking thread");
                loop {
                    log::debug!("Before msg!");
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
            })
        };
        task.await.unwrap()
    }
    // fn new() -> Self {
    //     Self {}
    // }
}
