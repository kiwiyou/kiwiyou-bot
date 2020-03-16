use super::{command, Text};
use crate::service::unicode::LookupResult;
use teloxide::utils::html::{bold, code_inline, escape, italic};

pub struct English;

impl Text<English> for command::Result {
    fn to(&self, _: English) -> String {
        match self {
            command::Result::Help => format!(
                "👉 {title} \n\
                    {help} displays this help message. \n\
                    {usearch} searches for the unicode character. \n\
                    {language} changes the bot's language.",
                title = bold("Usage"),
                help = code_inline("/help"),
                usearch = code_inline("/usearch"),
                language = code_inline("/lang"),
            ),
            command::Result::Language { before, after } => {
                let before_text = before.map_or("(None)".into(), |k| k.to_string());
                format!(
                    "Language for this chat has changed! \n\
                    Before: {before} \n\
                    After: {after}",
                    before = before_text,
                    after = bold(&after.to_string()),
                )
            }
            command::Result::UnicodeSearch(result) => {
                let records = match result {
                    LookupResult::List(list) => list
                        .characters
                        .iter()
                        .map(|record| {
                            let ancillary = if record.old_name.is_empty() {
                                "".to_string()
                            } else {
                                format!(" ({})", escape(&record.old_name))
                            };

                            format!(
                                "\u{200E}{character} {name}{ancillary}",
                                character = code_inline(&record.character),
                                name = escape(&record.name),
                                ancillary = ancillary,
                            )
                        })
                        .collect(),
                    LookupResult::Single(single) => vec![format!(
                        "\u{200E}{character} {name}",
                        character = code_inline(&single.character),
                        name = escape(&single.name),
                    )],
                };

                format!(
                    "☑️ {title} \n\n\
                    {result}",
                    title = bold("Results"),
                    result = records.join("\n")
                )
            }
        }
    }
}

impl Text<English> for command::Error {
    fn to(&self, language: English) -> String {
        match self {
            command::Error::NotFound => command::Result::Help.to(language),
            command::Error::Usage(usage) => usage.to(language),
            command::Error::Exception => "An error occurred to the bot. Try again later.".into(),
        }
    }
}

impl Text<English> for command::Usage {
    fn to(&self, _: English) -> String {
        match self {
            command::Usage::UnicodeSearch => format!(
                "👉 Usage of {command} \n\
                    {full_command} \n\
                    searches for the unicode character whose name contains keyword {keyword}.",
                command = code_inline("/usearch"),
                full_command = code_inline("/usearch <keyword>"),
                keyword = italic(&escape("<keyword>")),
            ),
            command::Usage::Language => format!(
                "👉 Usage of {command} \n\
                    {full_command} \n\
                    changes the bot's language in this chat to {language}. \n\
                    {supported}",
                command = code_inline("/lang"),
                full_command = code_inline("/lang <language>"),
                language = italic(&escape("<language>")),
                supported = italic("Currently supports: English"),
            ),
        }
    }
}
