use super::{command, Language};
use crate::service::unicode::LookupResult;
use teloxide::utils::html::{bold, code_inline, escape, italic};

pub struct English;

impl Language<command::Result> for English {
    fn equivalent_of(&self, text: command::Result) -> String {
        match text {
            command::Result::Help => format!(
                "👉 {title} \n\
                    {help} displays this help message. \n\
                    {usearch} searches for the unicode character.",
                title = bold("Usage"),
                help = code_inline("/help"),
                usearch = code_inline("/usearch")
            ),
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

impl Language<command::Error> for English {
    fn equivalent_of(&self, text: command::Error) -> String {
        match text {
            command::Error::NotFound => self.equivalent_of(command::Result::Help),
            command::Error::Usage(usage) => self.equivalent_of(usage),
            command::Error::Exception => "An error occurred to the bot. Try again later.".into(),
        }
    }
}

impl Language<command::Usage> for English {
    fn equivalent_of(&self, text: command::Usage) -> String {
        match text {
            command::Usage::UnicodeSearch => format!(
                "👉 Usage of {command} \n\
                    {full_command} \n\
                    searches for the unicode character whose name contains keyword {keyword}.",
                command = code_inline("/usearch"),
                full_command = code_inline("/usearch <keyword>"),
                keyword = italic(&escape("<keyword>")),
            ),
        }
    }
}
