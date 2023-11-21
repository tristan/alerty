use itertools::izip;
use scraper::{Html, Selector};
use serde::Deserialize;
use crate::{AlertyError, AlertData, source_iter::{AlertSourceConfig, AlertSource}};

pub struct Bandwear {
    shop_name: String
}

#[derive(Deserialize)]
pub struct BandwearConfig {
    shop_name: String,
}

impl AlertSourceConfig for BandwearConfig {
}

impl AlertSource for Bandwear {
    type Config = BandwearConfig;

    fn initialize(config: &Self::Config) -> Self {
        Self { shop_name: config.shop_name.clone() }
    }

    fn id(&self) -> String {
        self.shop_name.clone()
    }

    fn fetch(&self) -> Result<Vec<AlertData>, AlertyError> {
        let shop_name = &self.shop_name;
        let res = ureq::get(&format!("https://shop.bandwear.com/collections/{shop_name}?sort_by=created-descending"))
            .call();
        let res = match res {
            Ok(res) => res,
            Err(e) => {
                return Err(AlertyError::other(format!("ERROR FETCHING DATA: {e}")));
            }
        };
        //res.into_reader()
        let html = match res.into_string() {
            Ok(html) => html,
            Err(e) => {
                return Err(AlertyError::other(format!("ERROR PARING HTML: {e}")));
            }
        };

        let doc = Html::parse_document(&html);
        let link_selector = Selector::parse("div.thumbnail > a").unwrap();
        let thumb_selector = Selector::parse("div.thumbnail > a > div.product_image > img").unwrap();
        let title_selector = Selector::parse("div.thumbnail > a > div.info > span.title").unwrap();
        let price_selector = Selector::parse("div.thumbnail > a > div.info > span.price > span").unwrap();
        let link_selection = doc.select(&link_selector);
        let thumb_selection = doc.select(&thumb_selector);
        let title_selection = doc.select(&title_selector);
        let price_selection = doc.select(&price_selector);
        izip!(link_selection, thumb_selection, title_selection, price_selection).map(|(link, thumb, title, price)| {
            let link = link.attr("href").map(|link| format!("https://shop.bandwear.com/{link}"));
            if let Some(link) = link {
                if let Some((_, id)) = link.rsplit_once('/') {
                    let id = id.to_string();
                    let thumbnail = thumb.attr("src").map(|thumb| {
                        if thumb.starts_with("//") {
                            format!("https:{thumb}")
                        } else {
                            thumb.to_string()
                        }
                    });
                    let title = title.text().collect::<String>();
                    let text = price.text().collect::<String>();
                    return Ok(AlertData {
                        id,
                        title: Some(title),
                        text: Some(text),
                        thumbnail,
                        link: Some(link),
                    })
                }
            }
            Err(AlertyError::Other("Unexpected html data".to_string()))

        }).collect::<Result<Vec<AlertData>, AlertyError>>()
    }
}
