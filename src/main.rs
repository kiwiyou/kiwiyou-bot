use lambda_http::{lambda, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::json;
use std::{cell::RefCell, sync::Arc};
use teloxide::{error_handlers::OnError, types::Update, types::UpdateKind, Bot};
use tokio::runtime::Runtime;

mod command;
mod database;
mod i18n;
mod service;
mod util;

thread_local! {
    static RUNTIME: RefCell<Runtime> =
        RefCell::new(Runtime::new().expect("unable to create runtime."));
}

pub struct MessageContext {
    pub bot: Arc<Bot>,
    pub language: Option<i18n::LanguageKind>,
}

fn main() {
    lambda!(handler)
}

fn handler(request: Request, _: Context) -> Result<impl IntoResponse, HandlerError> {
    let token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN is not set.");
    pretty_env_logger::init();
    let bot = Bot::new(token);
    let update: Update = serde_json::from_slice(request.body())?;
    let task = run(bot, update);
    RUNTIME.with(|rt| rt.borrow_mut().block_on(task));
    Ok(json! ({
        "statusCode": 200
    }))
}

async fn run(bot: Arc<Bot>, update: Update) {
    if let UpdateKind::Message(message) = update.kind {
        let language = database::get_language(message.chat_id()).await;
        let context = MessageContext {
            bot,
            language: language.unwrap_or(None),
        };
        command::handle_message(message, context)
            .await
            .log_on_error()
            .await;
    }
}
