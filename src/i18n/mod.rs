mod english;
pub use english::*;

pub trait Language<T> {
    fn equivalent_of(&self, text: T) -> String;
}

pub mod command {
    pub enum Result {
        Help,
        UnicodeSearch(crate::service::unicode::LookupResult),
    }

    pub enum Error {
        NotFound,
        Usage(Usage),
        Exception,
    }

    pub enum Usage {
        UnicodeSearch,
    }
}
