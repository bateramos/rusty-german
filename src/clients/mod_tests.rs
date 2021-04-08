use std::borrow::Cow;
use std::collections::HashMap;

use typed_html::html;
use typed_html::dom::DOMTree;
use super::online_dictionary::fetch_phrases_from;

lazy_static! {
    static ref MOCK_HTML : HashMap<&'static str, String> = {
        [prepare_one_phrase_test(), prepare_two_phrase_test()].iter().cloned().collect()
    };
}

pub async fn fetch(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(MOCK_HTML.get(url).unwrap().to_string())
}

pub fn create_url<'a>(_domain: &str, path: &'a str) -> Cow<'a, str> {
    path.into()
}

fn prepare_one_phrase_test() -> (&'static str, String) {
    let doc : DOMTree<String> = html!(
        <html>
            <head><title>"Title"</title></head>
            <body>
                <div id="section-example">
                    <samp>
                        "This is a text"
                    </samp>
                </div>
            </body>
        </html>
    );

    ("one_phrase_test", doc.to_string())
}

fn prepare_two_phrase_test() -> (&'static str, String) {
    let doc : DOMTree<String> = html!(
        <html>
            <head><title>"Title"</title></head>
            <body>
                <div id="section-example">
                    <samp>"This is a text"</samp>
                    <samp>"This is a text 2"</samp>
                </div>
            </body>
        </html>
    );

    ("two_phrase_test", doc.to_string())
}

#[test]
fn it_should_one_phrase_test() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let response = rt.block_on(async {
        fetch_phrases_from("one_phrase_test").await
    });

    assert_eq!(response.unwrap(), ["This is a text"]);
}

#[test]
fn it_should_two_phrase_test() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let response = rt.block_on(async {
        fetch_phrases_from("two_phrase_test").await
    });

    assert_eq!(response.unwrap(), ["This is a text", "This is a text 2"]);
}
