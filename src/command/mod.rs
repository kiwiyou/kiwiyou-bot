use crate::i18n::{self, English, Language};
use log::error;
use std::sync::Arc;
use teloxide::requests::{Request, SendChatActionKind};
use teloxide::types::{Message, ParseMode};
use teloxide::Bot;
use teloxide::RequestError;

pub async fn handle_message(bot: Arc<Bot>, message: Message) -> Result<(), RequestError> {
    if let Some(text) = message.text() {
        let without_slash = text.trim_start_matches('/');
        if without_slash.len() == text.len() {
            return Ok(());
        }
        bot.send_chat_action(message.chat_id(), SendChatActionKind::Typing)
            .send()
            .await?;
        let content = match Command::parse(without_slash) {
            Ok(command) => match command {
                Command::Help => English.equivalent_of(i18n::command::Result::Help),
                Command::UnicodeSearch { query } => {
                    match crate::service::unicode::lookup(query).await {
                        Ok(result) => {
                            English.equivalent_of(i18n::command::Result::UnicodeSearch(result))
                        }
                        Err(e) => {
                            error!("{:?} : {:?}", command, e);
                            English.equivalent_of(i18n::command::Error::Exception)
                        }
                    }
                }
            },
            Err(e) => English.equivalent_of(e),
        };

        bot.send_message(message.chat_id(), content)
            .parse_mode(ParseMode::HTML)
            .reply_to_message_id(message.id)
            .send()
            .await?;
    }
    Ok(())
}

#[derive(Debug)]
enum Command<'a> {
    Help,
    UnicodeSearch { query: &'a str },
}

impl<'a> Command<'a> {
    pub fn parse(text: &'a str) -> Result<Command<'a>, i18n::command::Error> {
        let mut arguments = crate::util::split_args(text);
        let label = arguments.next().ok_or(i18n::command::Error::NotFound)?;
        match label.trim_end_matches(&std::env::var("BOT_NAME").expect("BOT_NAME is not set.")) {
            "help" => Ok(Command::Help),
            "usearch" => {
                let query = arguments.rest();
                if query.is_empty() {
                    Err(i18n::command::Error::Usage(
                        i18n::command::Usage::UnicodeSearch,
                    ))
                } else {
                    Ok(Command::UnicodeSearch { query })
                }
            }
            _ => Err(i18n::command::Error::NotFound),
        }
    }
}
