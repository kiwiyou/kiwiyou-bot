mod english;
pub use english::*;

#[derive(Debug, Clone, Copy)]
pub enum LanguageKind {
    English,
}

impl std::fmt::Display for LanguageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::English => write!(f, "English"),
        }
    }
}

impl std::str::FromStr for LanguageKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "English" => Ok(Self::English),
            _ => Err(()),
        }
    }
}

impl Default for LanguageKind {
    fn default() -> Self {
        Self::English
    }
}

pub trait Text<T> {
    fn to(&self, language: T) -> String;
}

impl<T> Text<LanguageKind> for T
where
    T: Text<English>,
{
    fn to(&self, language: LanguageKind) -> String {
        match language {
            LanguageKind::English => self.to(English),
        }
    }
}

pub mod command {
    pub enum Result {
        Help,
        UnicodeSearch(crate::service::unicode::LookupResult),
        Language {
            before: Option<crate::i18n::LanguageKind>,
            after: crate::i18n::LanguageKind,
        },
    }

    pub enum Error {
        NotFound,
        Usage(Usage),
        Exception,
    }

    pub enum Usage {
        UnicodeSearch,
        Language,
    }
}
