use scraper::Html;
use scraper::Selector;

fn main() {
    for year in 2014..=2021 {
        for month in 1..=12 {
            let url = format!(
                "https://gihyo.jp/magazine/SD/archive/{}/{}{:0>2}",
                year, year, month
            );
            println!("{}", url);

            let body = reqwest::blocking::get(url).unwrap().text().unwrap();
            let document = Html::parse_document(&body);
            let selector = Selector::parse(r#"#toc > .readingContent01 > h3"#).unwrap();
            for element in document.select(&selector) {
                if let Some(title) = element
                    .select(&Selector::parse(r#".title"#).unwrap())
                    .next()
                {
                    println!("{:?}", title.inner_html())
                };
            }
        }
    }
}
