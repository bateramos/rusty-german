use crate::types::Article;

fn create_articles(case: &'static str, articles: [&'static str; 4]) -> Vec<Article> {
    let mut index = 0;
    let genders = ["Masculine", "Feminine", "Neutral", "Plural"];
    articles.iter().map(|name| {
        let article = Article { name, case, gender: genders[index] };
        index += 1;
        article
    }).collect::<Vec<Article>>()
}

pub fn get_articles() -> [Vec<Article>; 4] {
    [
        create_articles("Nominativ", ["der", "die", "das", "die"]),
        create_articles("Akkusativ", ["den", "die", "das", "die"]),
        create_articles("Dativ", ["dem", "der", "dem", "den"]),
        create_articles("Genitiv", ["des", "der", "des", "der"])
    ]
}
