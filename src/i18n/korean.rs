use super::{command, Text};
use crate::service::unicode::LookupResult;
use teloxide::utils::html::{bold, code_inline, escape, italic};

pub struct Korean;

impl Text<Korean> for command::Result {
    fn to(&self, _: Korean) -> String {
        match self {
            command::Result::Help => format!(
                "👉 {title} \n\
                    {help} - 도움말을 봅니다. \n\
                    {usearch} - 유니코드 상에서 문자를 검색합니다. \n\
                    {language} - 봇의 언어를 변경합니다.",
                title = bold("사용법"),
                help = code_inline("/help"),
                usearch = code_inline("/usearch"),
                language = code_inline("/lang"),
            ),
            command::Result::Language { before, after } => {
                let before_text = before.map_or("(미설정)".into(), |k| k.to_string());
                format!(
                    "언어를 변경했습니다! \n\
                    전: {before} \n\
                    후: {after}",
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
                    title = bold("검색 결과"),
                    result = records.join("\n")
                )
            }
        }
    }
}

impl Text<Korean> for command::Error {
    fn to(&self, language: Korean) -> String {
        match self {
            command::Error::NotFound => command::Result::Help.to(language),
            command::Error::Usage(usage) => usage.to(language),
            command::Error::Exception => "오류가 발생했습니다. 나중에 다시 시도해 주세요.".into(),
        }
    }
}

impl Text<Korean> for command::Usage {
    fn to(&self, _: Korean) -> String {
        match self {
            command::Usage::UnicodeSearch => format!(
                "👉 {command} 사용법 \n\
                    {full_command} \n\
                    이름에 {keyword}가 포함된 유니코드 문자를 검색합니다.",
                command = code_inline("/usearch"),
                full_command = code_inline("/usearch <검색어>"),
                keyword = escape("<검색어>"),
            ),
            command::Usage::Language => format!(
                "👉 {command} 사용법 \n\
                    {full_command} \n\
                    봇의 언어를 {language}로 설정합니다. \n\
                    {supported}",
                command = code_inline("/lang"),
                full_command = code_inline("/lang <언어>"),
                language = italic(&escape("<언어>")),
                supported = italic("현재 지원하는 언어: English, Korean"),
            ),
        }
    }
}
