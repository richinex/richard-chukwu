



use yew::{function_component, html, use_state, Callback, Html};
use yew::{prelude::*, virtual_dom::VNode};
use pulldown_cmark::{Parser, Options, html::push_html};

#[derive(Properties, PartialEq, Clone)]
pub struct ProjectItemProps {
    pub name: String,
    pub description: &'static str,
    pub github_link: String,
}

#[function_component(ProjectItem)]
pub fn project_item(ProjectItemProps { name, description, github_link }: &ProjectItemProps) -> Html {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(description, options);

    let mut html_output = String::new();
    push_html(&mut html_output, parser);

    let rendered_html = VNode::from_html_unchecked(html_output.into());

    html! {
        <div class="project-item transition-transform duration-300 ease-in-out">

            <h3 class="text-lg font-semibold text-center mb-4">{ name.clone() }</h3>
            <div class="content mb-4">{ rendered_html }</div>
            <a href={github_link.clone()} target="_blank" class="github-link text-blue-500 hover:text-blue-700 transition-colors duration-300">{"View Project"}</a>
        </div>
    }
}

#[function_component(Projects)]
pub fn projects() -> Html {
    let projects = vec![
        ProjectItemProps {
            name: "FitMyEIS".to_string(),
            description: include_str!("../../static/markdown/projects/fitmyeis.md"),
            github_link: "https://www.fitmyeis.com".to_string(),
        },
        ProjectItemProps {
            name: "PyMultiplEIS".to_string(),
            description: include_str!("../../static/markdown/projects/pymultipleis.md"),
            github_link: "https://github.com/richinex/pymultipleis".to_string(),
        },
        ProjectItemProps {
            name: "Plexisort".to_string(),
            description: include_str!("../../static/markdown/projects/plexisort.md"),
            github_link: "https://github.com/richinex/plexisort".to_string(),
        },
        // Add more projects as needed
    ];
    let current_index = use_state(|| 0);
    let max_index = projects.len() - 1;

    let on_next = {
        let current_index = current_index.clone();
        Callback::from(move |_| {
            let new_index = (*current_index + 1).min(max_index);
            current_index.set(new_index);
        })
    };

    let on_prev = {
        let current_index = current_index.clone();
        Callback::from(move |_| {
            let new_index = (*current_index).checked_sub(1).unwrap_or(0);
            current_index.set(new_index);
        })
    };

    html! {
        <div class="projects-container max-w-7xl mx-auto mt-8 px-4">
        // <p class="text-center mb-4">{ "Description of some of my favorite projects." }</p>
            <div class="projects-slider">
                { for projects.iter().enumerate().filter_map(|(i, project)| {
                    if i == *current_index {
                        Some(html_nested!{ <ProjectItem ..project.clone() /> })
                    } else {
                        None
                    }
                })}
            </div>
            <div class="projects-navigation">
            <button onclick={on_prev} class="project-button">{ "Prev" }</button>
            <button onclick={on_next} class="project-button">{ "Next" }</button>
        </div>
        </div>
    }
}
