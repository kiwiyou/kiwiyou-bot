use crate::i18n::{self, Text};
use crate::MessageContext;
use log::error;
use teloxide::requests::{Request, SendChatActionKind};
use teloxide::types::{Message, ParseMode};
use teloxide::RequestError;

pub async fn handle_message(message: Message, ctx: MessageContext) -> Result<(), RequestError> {
    if let Some(text) = message.text() {
        let without_slash = text.trim_start_matches('/');
        if without_slash.len() == text.len() {
            return Ok(());
        }
        ctx.bot
            .send_chat_action(message.chat_id(), SendChatActionKind::Typing)
            .send()
            .await?;
        let language = ctx.language.unwrap_or_default();
        let content = match Command::parse(without_slash) {
            Ok(command) => match command {
                Command::Help => i18n::command::Result::Help.to(language),
                Command::UnicodeSearch { query } => {
                    match crate::service::unicode::lookup(query).await {
                        Ok(result) => i18n::command::Result::UnicodeSearch(result).to(language),
                        Err(e) => {
                            error!("{:?} : {:?}", command, e);
                            i18n::command::Error::Exception.to(language)
                        }
                    }
                }
                Command::Language { kind } => {
                    let set_language = crate::database::set_language(message.chat_id(), kind).await;
                    match set_language {
                        Ok(_) => i18n::command::Result::Language {
                            before: ctx.language,
                            after: kind,
                        }
                        .to(kind),
                        Err(e) => {
                            error!("{:?} : {:?}", command, e);
                            i18n::command::Error::Exception.to(language)
                        }
                    }
                }
            },
            Err(e) => e.to(language),
        };

        ctx.bot
            .send_message(message.chat_id(), content)
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
    Language { kind: i18n::LanguageKind },
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
            "lang" => {
                let language = arguments.rest();
                if let Ok(kind) = language.parse() {
                    Ok(Command::Language { kind })
                } else {
                    Err(i18n::command::Error::Usage(i18n::command::Usage::Language))
                }
            }
            _ => Err(i18n::command::Error::NotFound),
        }
    }
}
