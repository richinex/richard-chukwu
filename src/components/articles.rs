// use yew::{prelude::*, virtual_dom::VNode};
// use pulldown_cmark::{Parser, Options, html::push_html};
// use chrono::NaiveDate;
// use std::collections::HashMap;
// use std::cmp::Reverse;
// // https://stackoverflow.com/questions/60916194/how-to-sort-a-vector-in-descending-order-in-rust

// #[derive(Properties, PartialEq, Clone)]
// pub struct ArticleItemProps {
//     pub title: String,
//     pub content: &'static str,
//     pub filename: String,
// }

// fn extract_date_from_filename(filename: &str) -> Result<NaiveDate, String> {
//     let parts: Vec<&str> = filename.split('_').collect();
//     if parts.len() > 2 {
//         let date_part = parts[2].split('.').next().unwrap();
//         NaiveDate::parse_from_str(date_part, "%d-%m-%Y")
//             .map_err(|e| e.to_string())
//     } else {
//         Err("Invalid filename format".to_string())
//     }
// }

// #[function_component(Articles)]
// pub fn articles() -> Html {
//     let articles = vec![
//         ArticleItemProps {
//             title: "Trait Objects vs Generics in Rust".to_string(),
//             content: include_str!("../../static/markdown/articles/article_01_15-04-2024.md"),
//             filename: "article_01_15-04-2024.md".to_string(),
//         },
//         ArticleItemProps {
//             title: "Learning Rust — A Guide to Great Resources".to_string(),
//             content: include_str!("../../static/markdown/articles/article_01_16-04-2024.md"),
//             filename: "article_01_16-04-2024.md".to_string(),
//         },
//     ];

//     let mut articles_by_date: HashMap<NaiveDate, Vec<&ArticleItemProps>> = HashMap::new();
//     for article in &articles {
//         let date = extract_date_from_filename(&article.filename).unwrap();
//         articles_by_date.entry(date).or_insert_with(Vec::new).push(article);
//     }
//     let mut dates: Vec<_> = articles_by_date.keys().cloned().collect();
//     dates.sort_by_key(|w| Reverse(*w));

//     let selected_index = use_state(|| 0);
//     let flat_articles: Vec<(NaiveDate, &ArticleItemProps)> = dates.iter()
//         .flat_map(|date| articles_by_date.get(date).unwrap().iter().map(move |article| (*date, *article)))
//         .collect();

//     let select_article = {
//         let selected_index = selected_index.clone();
//         move |index: usize| {
//             selected_index.set(index);
//         }
//     };

//     html! {
//         <div class="articles-layout">
//             <aside class="navigation-pane">
//                 <ul>
//                     { for dates.iter().enumerate().map(|(_, date)| {
//                         html! {
//                             <>
//                                 <li class="date-header">{ date.format("%d-%m-%Y").to_string() }</li>
//                                 { for articles_by_date.get(date).unwrap().iter().enumerate().map(|(_, article)| {
//                                     let global_index = flat_articles.iter().position(|(_, a)| *a == *article).unwrap();
//                                     let onclick = {
//                                         let select_article = select_article.clone();
//                                         Callback::from(move |_| select_article(global_index))
//                                     };
//                                     let class = if *selected_index == global_index { "active" } else { "" };
//                                     html! {
//                                         <li {onclick} class={classes!(class)}>
//                                             { &article.title }
//                                         </li>
//                                     }
//                                 })}
//                             </>
//                         }
//                     })}
//                 </ul>
//             </aside>
//             <article class="article-content">
//                 { if let Some((_, article)) = flat_articles.get(*selected_index) {
//                     let mut options = Options::empty();
//                     options.insert(Options::ENABLE_STRIKETHROUGH);
//                     options.insert(Options::ENABLE_TABLES);  // Enable tables
//                     options.insert(Options::ENABLE_HEADING_ATTRIBUTES);  // Enable heading attributes
//                     let parser = Parser::new_ext(&article.content, options);

//                     let mut html_output = String::new();
//                     push_html(&mut html_output, parser);
//                     let rendered_content = VNode::from_html_unchecked(html_output.into());

//                     html! {
//                         <>
//                             <div class="content">{ rendered_content }</div>
//                         </>
//                     }
//                 } else {
//                     html! { <p>{"No article selected"}</p> }
//                 }}
//             </article>
//         </div>
//     }
// }

use yew::{prelude::*, virtual_dom::VNode};
use pulldown_cmark::{Parser, Options, html::push_html};
use chrono::NaiveDate;
use std::collections::HashMap;
use std::cmp::Reverse;

#[derive(Properties, PartialEq, Clone)]
pub struct ArticleItemProps {
    pub title: String,
    pub content: &'static str,
    pub filename: String,
}

fn extract_date_from_filename(filename: &str) -> Result<NaiveDate, String> {
    filename.split('_').nth(2)
        .and_then(|date_part| date_part.split('.').next())
        .and_then(|date_str| NaiveDate::parse_from_str(date_str, "%d-%m-%Y").ok())
        .ok_or_else(|| "Invalid filename format".to_string())
}

#[function_component(Articles)]
pub fn articles() -> Html {
    let articles = vec![
        ArticleItemProps {
            title: "Mastering Trait Objects vs Generics in Rust".to_string(),
            content: include_str!("../../static/markdown/articles/article_01_15-04-2024.md"),
            filename: "article_01_15-04-2024.md".to_string(),
        },
        ArticleItemProps {
            title: "The Ultimate Guide to Learning Rust".to_string(),
            content: include_str!("../../static/markdown/articles/article_01_16-04-2024.md"),
            filename: "article_01_16-04-2024.md".to_string(),
        },
    ];

    let articles_by_date: HashMap<_, _> = articles.iter()
        .filter_map(|article| {
            extract_date_from_filename(&article.filename)
                .ok()
                .map(|date| (date, article))
        })
        .fold(HashMap::new(), |mut acc, (date, article)| {
            acc.entry(date).or_insert_with(Vec::new).push(article);
            acc
        });

    let dates: Vec<_> = articles_by_date.keys().cloned().collect();
    let selected_index = use_state(|| 0);
    let flat_articles: Vec<_> = dates.iter()
        .rev()
        .flat_map(|date| articles_by_date.get(date).unwrap().iter().map(move |&article| (*date, article)))
        .collect();

    let select_article = {
        let selected_index = selected_index.clone();
        Callback::from(move |index: usize| selected_index.set(index))
    };

    html! {
        <div class="articles-layout">
            <aside class="navigation-pane">
                <ul>
                    { for dates.iter().rev().map(|date| {
                        html! {
                            <>
                                <li class="date-header">{ date.format("%d-%m-%Y").to_string() }</li>
                                { for articles_by_date.get(date).unwrap().iter().enumerate().map(|(_, &article)| {
                                    let global_index = flat_articles.iter().position(|(_, a)| *a == article).unwrap();
                                    let onclick = select_article.reform(move |_| global_index);
                                    let class = if *selected_index == global_index { "active" } else { "" };
                                    html! {
                                        <li {onclick} class={classes!(class)}>
                                            { &article.title }
                                        </li>
                                    }
                                })}
                            </>
                        }
                    })}
                </ul>
            </aside>
            <article class="article-content">
                { flat_articles.get(*selected_index).map_or_else(
                    || html! { <p>{"No article selected"}</p> },
                    |(_, article)| {
                        let mut options = Options::empty();
                        options.insert(Options::ENABLE_STRIKETHROUGH | Options::ENABLE_TABLES | Options::ENABLE_HEADING_ATTRIBUTES);
                        let parser = Parser::new_ext(&article.content, options);
                        let mut html_output = String::new();
                        push_html(&mut html_output, parser);
                        html! {
                            <div class="content">
                                { VNode::from_html_unchecked(html_output.into()) }
                            </div>
                        }
                    }
                )}
            </article>
        </div>
    }
}