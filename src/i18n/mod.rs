mod english;
mod korean;
pub use english::*;
pub use korean::*;

#[derive(Debug, Clone, Copy)]
pub enum LanguageKind {
    English,
    Korean,
}

impl std::fmt::Display for LanguageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::English => write!(f, "English"),
            Self::Korean => write!(f, "한국어"),
        }
    }
}

impl std::str::FromStr for LanguageKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "english" => Ok(Self::English),
            "korean" => Ok(Self::Korean),
            _ => Err(()),
        }
    }
}

impl Default for LanguageKind {
    fn default() -> Self {
        Self::English
    }
}

impl LanguageKind {
    pub fn canonical_name(&self) -> &'static str {
        match self {
            Self::English => "English",
            Self::Korean => "Korean",
        }
    }
}

pub trait Text<T> {
    fn to(&self, language: T) -> String;
}

impl<T> Text<LanguageKind> for T
where
    T: Text<English> + Text<Korean>,
{
    fn to(&self, language: LanguageKind) -> String {
        match language {
            LanguageKind::English => self.to(English),
            LanguageKind::Korean => self.to(Korean),
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
