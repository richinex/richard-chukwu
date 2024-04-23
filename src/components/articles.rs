/* use yew::prelude::*;
use pulldown_cmark::{Parser, Options, html::push_html};
use yew::{prelude::*, virtual_dom::VNode};
use web_sys::console;

#[derive(Properties, PartialEq, Clone)]
pub struct ArticleItemProps {
    pub title: String,
    pub content: &'static str,  // Assuming content is stored as markdown text
}

#[function_component(Articles)]
pub fn articles() -> Html {
    let articles = vec![
        ArticleItemProps {
            title: "Introduction to Yew".to_string(),
            content: include_str!("markdown/project1.md"),
        },
        ArticleItemProps {
            title: "Advanced Yew Techniques".to_string(),
            content: include_str!("markdown/article_01_23-04-2024.md"),
        },
        // Additional articles...
    ];
    let selected_index = use_state(|| 0);

    // Define a callback for selecting an article
    let select_article = {
        let selected_index = selected_index.clone();
        move |index: usize| {
            selected_index.set(index);
        }
    };

    let selected_article = articles.get(*selected_index).unwrap();

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(selected_article.content, options);

    let mut html_output = String::new();
    push_html(&mut html_output, parser);
    let rendered_content = VNode::from_html_unchecked(html_output.into());

     // Prepare to render the list of article titles
     html! {
        <div class="articles-layout">
            <aside class="navigation-pane">
                <ul>
                    { for articles.iter().enumerate().map(|(index, article)| {
                        // Clone the callback here for each list item
                        let onclick = {
                            let select_article = select_article.clone();
                            Callback::from(move |_| select_article(index))
                        };
                        html! {
                            <li onclick={onclick} class={classes!(if *selected_index == index { "active" } else { "" })}>{ &article.title }</li>
                        }

                    })}
                </ul>
            </aside>
            <article class="article-content">
                { if let Some(article) = articles.get(*selected_index) {
                    html! {
                        <>
                            <h1>{ &article.title }</h1>
                            <div class="content">{ rendered_content }</div>
                        </>
                    }
                } else {
                    html! { <p>{"No article selected"}</p> }
                }}
            </article>
        </div>
    }
}


 */

/* use yew::prelude::*;
use pulldown_cmark::{Parser, Options, html::push_html};
use chrono::{NaiveDate, format::ParseError};
use yew::{prelude::*, virtual_dom::VNode};

#[derive(Properties, PartialEq, Clone)]
pub struct ArticleItemProps {
    pub title: String,
    pub content: &'static str,
    pub filename: String,
}

/// Extracts the date from the filename in the format `article_XX_DD-MM-YYYY.md`
fn extract_date_from_filename(filename: &str) -> Result<NaiveDate, String> {
    let parts: Vec<&str> = filename.split('_').collect();
    if parts.len() > 2 {
        let date_part = parts[2].split('.').next().unwrap(); // Safely get the date part before ".md"
        NaiveDate::parse_from_str(date_part, "%d-%m-%Y")
            .map_err(|e| e.to_string()) // Convert the parse error to a String for easier handling
    } else {
        Err("Invalid filename format".to_string())
    }
}


#[function_component(Articles)]
pub fn articles() -> Html {
    let mut articles = vec![
        ArticleItemProps {
            title: "Introduction to Yew".to_string(),
            content: include_str!("markdown/article_01_23-04-2024.md"),
            filename: "article_01_23-04-2024.md".to_string(),
        },
        ArticleItemProps {
            title: "Advanced Yew Techniques".to_string(),
            content: include_str!("markdown/article_01_23-04-2024.md"),
            filename: "article_02_23-04-2024.md".to_string(),
        },
        // More articles can be added here...
    ];

    // Sort articles by date extracted from filename
    articles.sort_by(|a, b| {
        let date_a = extract_date_from_filename(&a.filename).unwrap();
        let date_b = extract_date_from_filename(&b.filename).unwrap();
        date_a.cmp(&date_b)
    });

    let selected_index = use_state(|| 0);

    let select_article = {
        let selected_index = selected_index.clone();
        move |index: usize| {
            selected_index.set(index);
        }
    };

    html! {
        <div class="articles-layout">
            <aside class="navigation-pane">
                <ul>
                    { for articles.iter().enumerate().map(|(index, article)| {
                        let onclick = {
                            let select_article = select_article.clone();
                            Callback::from(move |_| select_article(index))
                        };
                        let class = if *selected_index == index { "active" } else { "" };
                        html! {
                            <li {onclick} class={classes!(class)}>
                                { format!("{} - {}", article.title, extract_date_from_filename(&article.filename).unwrap().format("%d-%m-%Y")) }
                            </li>
                        }
                    })}
                </ul>
            </aside>
            <article class="article-content">
                { if let Some(article) = articles.get(*selected_index) {
                    let mut options = Options::empty();
                    options.insert(Options::ENABLE_STRIKETHROUGH);
                    let parser = Parser::new_ext(&article.content, options);

                    let mut html_output = String::new();
                    push_html(&mut html_output, parser);
                    let rendered_content = VNode::from_html_unchecked(html_output.into());

                    html! {
                        <>
                            <h1>{ &article.title }</h1>
                            <div class="content">{ rendered_content }</div>
                        </>
                    }
                } else {
                    html! { <p>{"No article selected"}</p> }
                }}
            </article>
        </div>
    }
}
 */

use yew::{prelude::*, virtual_dom::VNode};
use pulldown_cmark::{Parser, Options, html::push_html};
use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(Properties, PartialEq, Clone)]
pub struct ArticleItemProps {
    pub title: String,
    pub content: &'static str,
    pub filename: String,
}

fn extract_date_from_filename(filename: &str) -> Result<NaiveDate, String> {
    let parts: Vec<&str> = filename.split('_').collect();
    if parts.len() > 2 {
        let date_part = parts[2].split('.').next().unwrap();
        NaiveDate::parse_from_str(date_part, "%d-%m-%Y")
            .map_err(|e| e.to_string())
    } else {
        Err("Invalid filename format".to_string())
    }
}

#[function_component(Articles)]
pub fn articles() -> Html {
    let articles = vec![
        ArticleItemProps {
            title: "Introduction to Yew".to_string(),
            content: include_str!("../../static/markdown/articles/article_01_23-04-2024.md"),
            filename: "article_01_23-04-2024.md".to_string(),
        },
        ArticleItemProps {
            title: "Advanced Yew Techniques".to_string(),
            content: include_str!("../../static/markdown/articles/article_01_23-04-2024.md"),
            filename: "article_02_23-04-2024.md".to_string(),
        },
        // More articles can be added here...
    ];

    let mut articles_by_date: HashMap<NaiveDate, Vec<&ArticleItemProps>> = HashMap::new();
    for article in &articles {
        let date = extract_date_from_filename(&article.filename).unwrap(); // Assume unwrap for simplicity
        articles_by_date.entry(date).or_insert_with(Vec::new).push(article);
    }

    let mut dates: Vec<_> = articles_by_date.keys().cloned().collect();
    dates.sort(); // Sort dates

    let selected_index = use_state(|| 0);

    let select_article = {
        let selected_index = selected_index.clone();
        move |index: usize| {
            selected_index.set(index);
        }
    };

    html! {
        <div class="articles-layout">
            <aside class="navigation-pane">
                <ul>
                    { for dates.iter().map(|date| {
                        html! {
                            <>
                                <li class="date-header">{ date.format("%d-%m-%Y").to_string() }</li>
                                { for articles_by_date.get(date).unwrap().iter().enumerate().map(|(index, article)| {
                                    let onclick = {
                                        let select_article = select_article.clone();
                                        Callback::from(move |_| select_article(index))
                                    };
                                    let class = if *selected_index == index { "active" } else { "" };
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
                { if let Some(article) = articles.get(*selected_index) {
                    let mut options = Options::empty();
                    options.insert(Options::ENABLE_STRIKETHROUGH);
                    let parser = Parser::new_ext(&article.content, options);

                    let mut html_output = String::new();
                    push_html(&mut html_output, parser);
                    let rendered_content = VNode::from_html_unchecked(html_output.into());

                    html! {
                        <>
                            <h1>{ &article.title }</h1>
                            <div class="content">{ rendered_content }</div>
                        </>
                    }
                } else {
                    html! { <p>{"No article selected"}</p> }
                }}
            </article>
        </div>
    }
}
