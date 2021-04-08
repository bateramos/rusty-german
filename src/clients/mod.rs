#[cfg(not(test))]
use std::borrow::Cow;

#[cfg(not(test))]
async fn fetch(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(reqwest::get(url)
        .await?
        .text()
        .await?)
}

#[cfg(not(test))]
fn create_url<'a>(domain: &str, path: &str) -> Cow<'a, str> {
    format!("{}{}", domain, path).into()
}

pub mod online_dictionary {
    static URL : &str = "https://dict.leo.org/englisch-deutsch/";

    use select::predicate::{Attr, Name};
    use select::document::Document;

    pub async fn fetch_phrases_from(word: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let html = super::fetch(&super::create_url(URL, word)).await?;

        let document = Document::from(&html[..]);

        let mut phrases : Vec<String> = vec![];
        for node in document.find(Attr("id", "section-example")) {
            for samp in node.find(Name("samp")) {
                phrases.push(samp.text().trim().replace("AE", "").replace("BE", "").to_string());
            }
        }

        Ok(phrases)
    }
}

#[cfg(test)]
#[path = "mod_tests.rs"]
mod tests;
#[cfg(test)]
use tests::{fetch, create_url};
