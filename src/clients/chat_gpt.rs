use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Serialize, Deserialize};

static BASE_URL : &str = "https://api.openai.com";

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct RequestBody {
    model: String,
    messages: Vec<Message>
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct ResponseBody {
    choices: Vec<Choice>
}

impl RequestBody {
    fn new(verb: String) -> Self {
        let messages = vec![
            Message { role: "system".to_owned(), content: "
            You are a C1 German teacher.
            You will receive a German verb from the student.
            You will give back a phrase using this verb in English for the student to translate into German.
            You will say true to correct translation or false with some detailed tips for student to try again if it is incorrect.
            ".to_owned()},
            Message { role: "user".to_owned(), content: verb },
        ];
        //RequestBody { model: "gpt-3.5-turbo".to_owned(), messages }
        RequestBody { model: "gpt-4o".to_owned(), messages }
    }

    fn new_with_response(verb: String, phrase: String, translation: String) -> Self {
        let mut request_body = Self::new(verb);
        request_body.messages.push(Message { role: "assistant".to_owned(), content: phrase.to_owned() });
        request_body.messages.push(Message { role: "user".to_owned(), content: translation.to_owned() });

        request_body
    }
}

async fn fetch<'a>(request_body: RequestBody) -> Result<ResponseBody, Box<dyn std::error::Error>> {
    let chat_gpt_key = std::env::var("CHAT_GPT_KEY").unwrap_or_else(|_| {
        panic!("Missing CHAT_GPT_KEY environment variable");
    });

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let client = reqwest::Client::new();
    let req = client
        .post(format!("{}/v1/chat/completions", BASE_URL))
        .headers(headers)
        .bearer_auth(chat_gpt_key)
        .json(&request_body)
        .build()?;

    Ok(client.execute(req)
        .await?
        .json()
        .await?)
}

pub async fn fetch_phrase_for(verb: String) -> Result<String, Box<dyn std::error::Error>> {
    let result = fetch(RequestBody::new(verb)).await?;
    Ok(result.choices[0].message.content.to_string())
}

pub async fn verify_translation(verb: String, phrase: &String, translation: String) -> Result<String, Box<dyn std::error::Error>> {
    let result = fetch(RequestBody::new_with_response(verb, phrase.to_string(), translation)).await?;
    Ok(result.choices[0].message.content.to_string())
}
