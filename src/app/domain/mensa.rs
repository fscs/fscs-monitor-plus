use leptos::{prelude::ServerFnError, server};
use reqwest::Url;
use scraper::{node::Text, ElementRef, Selector};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Mensa {
    pub name: String,
    pub speiseplan: Vec<Essen>,
    pub id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Essen {
    pub name: String,
    pub vegan: bool,
    pub thumbnail: String,
}

impl Mensa {
    pub fn default() -> Self {
        Mensa {
            name: String::new(),
            speiseplan: Vec::new(),
            id: "".to_string(),
        }
    }
}

#[server]
pub async fn get_essen(id: String) -> Result<Vec<Essen>, ServerFnError> {
    let url = format!("https://www.stw-d.de/en/speiseplaene/{}/", id);
    let data = reqwest::get(url).await?;

    let html = data.text().await?;

    let document = scraper::Html::parse_fragment(&html);

    let selector = scraper::Selector::parse("div.counter-table").unwrap();

    let date_selector = format!(
        r#"div[data-date="{}"]"#,
        chrono::Local::now().format("%d.%m.%Y").to_string()
    );
    let date_selector = Selector::parse(&date_selector).unwrap();
    let name_selector = scraper::Selector::parse("ul").unwrap();
    let thumbnail_selector = scraper::Selector::parse("div.thumbnail").unwrap();

    let mut essen = Vec::new();
    document
        .select(&date_selector)
        .next()
        .unwrap()
        .attr("data-date")
        .unwrap()
        .to_string()
        .contains(&chrono::Local::now().format("%d.%m.%Y").to_string());

    if let Some(element) = document.select(&date_selector).next() {
        // Extract the inner HTML content of the selected element

        for element in element.select(&selector) {
            let mut name = String::new();
            let mut vegan = true;

            let mut thumbnail = String::new();
            for child in element.children() {
                if let Some(child_element) = ElementRef::wrap(child) {
                    if let Some(name_elem) = child_element.select(&name_selector).next() {
                        name = name_elem
                            .text()
                            .into_iter()
                            .skip(1)
                            .next()
                            .unwrap()
                            .to_string();

                        vegan = name_elem
                            .text()
                            .filter(|x| !x.to_string().trim().is_empty())
                            .all(|x| x.contains("[V]"));
                    }
                }
            }
            if let Some(thumbnail_elem) = element.select(&thumbnail_selector).next() {
                if let Some(attr) = thumbnail_elem.attr("style") {
                    thumbnail = attr
                        .strip_prefix("background-image: url(")
                        .unwrap()
                        .strip_suffix(")")
                        .unwrap()
                        .to_string();
                }
            }
            essen.push(Essen {
                name: name.clone(),
                vegan,
                thumbnail,
            });
        }
    }

    Ok(essen)
}
