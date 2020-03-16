use unhtml::FromHtml;
use unhtml_derive::*;

pub enum LookupResult {
    List(UnicodeList),
    Single(UnicodeSingle),
}

#[derive(FromHtml)]
#[html(selector = "tbody")]
pub struct UnicodeList {
    #[html(selector = "tr:not(.tablehead)")]
    pub characters: Vec<UnicodeInfo>,
}

#[derive(FromHtml)]
pub struct UnicodeInfo {
    #[html(selector = ".character", attr = "inner")]
    pub character: String,
    #[html(selector = ".name, .namelong", attr = "inner")]
    pub name: String,
    #[html(selector = ".oldname", attr = "inner", default = "")]
    pub old_name: String,
}

#[derive(FromHtml)]
#[html(selector = "tbody")]
pub struct UnicodeSingle {
    #[html(selector = ".onecharacter", attr = "inner")]
    pub character: String,
    #[html(selector = ".name", attr = "inner")]
    pub name: String,
}

pub async fn lookup(query: &str) -> Result<LookupResult, surf::Exception> {
    let html = surf::get(format!(
        "https://unicode-search.net/unicode-namesearch.pl?term={}&.submit=Search&print=1",
        query
    ))
    .recv_string()
    .await?;
    UnicodeList::from_html(&html)
        .map(LookupResult::List)
        .or_else(|_| UnicodeSingle::from_html(&html).map(LookupResult::Single))
        .map_err(|_| format!("Error parsing result for query={:?}", query).into())
}
