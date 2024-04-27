
use yew::prelude::*;
use web_sys::{self, HtmlInputElement, console};

#[derive(Clone, Debug)]
struct TriviaQuestion {
    query: &'static str,
    answer: &'static str,
}

const QUESTIONS: [TriviaQuestion; 10] = [
    TriviaQuestion { query: "Who called his elephant 'Tantor'?", answer: "Tarzan" },
    TriviaQuestion { query: "Which chess piece was originally called 'the elephant'?", answer: "Bishop" },
    TriviaQuestion { query: "Peanut butter, coffee, or roses — according to a Yale study, what's the number one most recognized smell among American adults?", answer: "Coffee" },
    TriviaQuestion { query: "What's the only land mammal taller than the elephant?", answer: "Giraffe" },
    TriviaQuestion { query: "Of the three substances that make up your teeth—enamel, dentin, and pulp—which is elephant ivory made of?", answer: "Dentin" },
    TriviaQuestion { query: "What word comes from the Greek for 'thick-skinned'?", answer: "Pachyderm" },
    TriviaQuestion { query: "As mystery fans know, cyanide is often said to have the bitter aroma of what?", answer: "Almonds" },
    TriviaQuestion { query: "What molecule is named for the Greek word for 'smell'?", answer: "Ozone" },
    TriviaQuestion { query: "What port's name comes from the Cantonese for 'fragrant harbor'?", answer: "Hong Kong" },
    TriviaQuestion { query: "What animal secretes ambergris?", answer: "Whale" },
];

fn normalize_answer(input: &str) -> String {
    input.trim().to_lowercase().replace(",", "").replace(".", "").replace("'", "")
}

#[function_component(TriviaQuiz)]
fn trivia_quiz() -> Html {
    let selected_question = use_state(|| QUESTIONS[0].clone());
    let selected_question_clone = selected_question.clone();
    let user_answer = use_state(|| String::new());
    let result_message = use_state(|| None::<String>);
    let result_effect_class = use_state(|| String::new());
    let current_index = use_state(|| 0);
    let score = use_state(|| 0);
    let quiz_complete = use_state(|| false);
    let answer_checked = use_state(|| false);

    let check_answer = {
        let user_answer = user_answer.clone();
        let selected_question = selected_question.clone();
        let result_message = result_message.clone();
        let result_effect_class = result_effect_class.clone();
        let answer_checked = answer_checked.clone();

        Callback::from(move |_| {
            if user_answer.trim().is_empty() {
                result_effect_class.set("warning".to_string());
                console::log_1(&"Please enter an answer.".into());
            } else {
                let correct = normalize_answer(&user_answer) == normalize_answer(selected_question.answer);
                if correct {
                    result_message.set(Some("Correct Answer!".to_string()));
                    result_effect_class.set("balloons".to_string());
                } else {
                    result_message.set(Some("Incorrect. Try again!".to_string()));
                    result_effect_class.set("crash".to_string());
                }
                answer_checked.set(true);
            }
        })
    };

        let oninput = {
        let user_answer = user_answer.clone(); // Clone for the input handler
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            user_answer.set(input.value());
            console::log_1(&format!("User input: {}", input.value()).into());
        })
    };

    let next_question = {
        let current_index = current_index.clone();
        let selected_question = selected_question.clone();
        let user_answer = user_answer.clone();
        let answer_checked = answer_checked.clone();
        let score = score.clone();
        let result_message = result_message.clone();
        let result_effect_class = result_effect_class.clone();
        let quiz_complete = quiz_complete.clone();

        Callback::from(move |_| {
            if *answer_checked {
                let correct = normalize_answer(&user_answer) == normalize_answer(selected_question.answer);
                let new_score = if correct { *score + 1 } else { *score };

                if correct {
                    score.set(new_score); // Update the score state only if correct
                    console::log_1(&format!("Correct! Score: {}", new_score).into());
                    result_message.set(Some("Correct Answer!".to_string()));
                    result_effect_class.set("balloons".to_string());
                } else {
                    result_message.set(Some("Incorrect. Moving to next question.".to_string()));
                    result_effect_class.set("crash".to_string());
                }

                // Check if it's the last question
                if *current_index < QUESTIONS.len() - 1 {
                    let new_index = *current_index + 1;
                    current_index.set(new_index);
                    selected_question.set(QUESTIONS[new_index].clone());
                    user_answer.set(String::new());
                    result_message.set(None);
                    result_effect_class.set(String::new());
                    answer_checked.set(false);
                } else {
                    quiz_complete.set(true);
                    result_message.set(Some(format!("Quiz complete! Your score: {}/{}", new_score, QUESTIONS.len())));
                }
            } else {
                console::log_1(&"Please check your answer before moving to the next question.".into());
                result_message.set(Some("Please check your answer before moving to the next question.".to_string()));
            }
        })
    };



    let reset_quiz = {
        let current_index = current_index.clone();
        let selected_question = selected_question.clone();
        let user_answer = user_answer.clone();
        let score = score.clone();
        let quiz_complete = quiz_complete.clone();
        let answer_checked = answer_checked.clone();
        let result_message = result_message.clone();
        let result_effect_class = result_effect_class.clone();

        Callback::from(move |_| {
            current_index.set(0);
            selected_question.set(QUESTIONS[0].clone());
            user_answer.set(String::new());
            score.set(0);
            quiz_complete.set(false);
            answer_checked.set(false);
            result_message.set(None);
            result_effect_class.set(String::new());
        })
    };


    html! {
                <div class="card">
                    <h2 class="text-2xl font-semibold">{ "Trivia Quiz" }</h2>
                    <p>{ selected_question_clone.query }</p>
                    <input type="text"
                        placeholder="Your answer"
                        class="input"
                        value={(*user_answer).clone()}
                        oninput={oninput}
                    />
                    <button onclick={check_answer} class="button check-answer-button" disabled={*quiz_complete}>{ "Check Answer" }</button>
                    <button onclick={next_question} class="button next-question-button" disabled={ *quiz_complete}>{ "Next Question" }</button>
                    <button onclick={reset_quiz} class="button reset-button">{ "Reset Quiz" }</button>
                    if let Some(message) = &*result_message {
                        <p>{ message }</p>
                    }
                    <div class={classes!("result-effect", (*result_effect_class).clone())}>
                        {
                            if *result_effect_class == "balloons" {
                                html! { <p>{ "🎈🎈🎈" }</p> }
                            } else if *result_effect_class == "crash" {
                                html! { <p>{ "💥" }</p> }
                            } else if *result_effect_class == "warning" {
                                html! { <p>{ "⚠️ Please enter an answer and hit the Check Answer button." }</p> }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                </div>
            }
}

#[function_component(Entertainment)]
pub fn entertainment() -> Html {
    html! {
        <div class="entertainment-section">
            <h1 class="text-4xl font-bold text-center mb-8">{ "Entertainment Section" }</h1>
            <div class="flex flex-wrap justify-around">
                <TriviaQuiz />
            </div>
        </div>
    }
}