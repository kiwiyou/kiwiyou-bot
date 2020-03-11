use lambda_http::{lambda, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::json;
use teloxide::{Bot, types::Update};
use tokio::runtime::Runtime;
use std::{sync::Arc, cell::RefCell};

thread_local! {
    static RUNTIME: RefCell<Runtime> = 
        RefCell::new(Runtime::new().expect("unable to create runtime."));
}

fn main() {
    lambda!(handler)
}

fn handler(
    request: Request,
    _: Context,
) -> Result<impl IntoResponse, HandlerError> {
    let token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN is not set.");
    if request.uri().path().ends_with(&token) {
        let bot = Bot::new(token);
        let update: Update = serde_json::from_slice(request.body())?;
        let task = run(bot, update);
        RUNTIME.with(|rt| rt.borrow_mut().block_on(task))?;
        Ok(json! ({
            "statusCode": 200
        }))
    } else {
        Ok(json! ({
            "statusCode": 403
        }))
    }
}

async fn run(_bot: Arc<Bot>, _update: Update) -> Result<(), HandlerError> {
    Ok(())
}