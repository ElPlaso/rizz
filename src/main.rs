use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct TriviaQuestion {
    response_code: f64,
    results: [TriviaResult; 1],
}

#[derive(Serialize, Deserialize, Debug)]
struct TriviaResult {
    question: String,
    correct_answer: String,
}

impl TriviaQuestion {
    async fn get(
        category: &String,
        difficulty: &String,
        question_type: &String,
    ) -> Result<Self, ExitFailure> {
        let url = format!(
            "https://opentdb.com/api.php?amount=1&category={}&difficulty={}&type={}",
            category, difficulty, question_type
        );

        let url = Url::parse(&*url)?;
        let res = reqwest::get(url).await?.json::<TriviaQuestion>().await?;

        Ok(res)
    }
}

enum TrueOrFalseAnswer {
    T,
    F,
}

impl TryFrom<String> for TrueOrFalseAnswer {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value {
            value if value == String::from("T") => Ok(TrueOrFalseAnswer::T),
            value if value == String::from("F") => Ok(TrueOrFalseAnswer::F),
            _ => Err(()),
        }
    }
}

impl From<TrueOrFalseAnswer> for String {
    fn from(answer: TrueOrFalseAnswer) -> Self {
        match answer {
            TrueOrFalseAnswer::T => String::from("True"),
            TrueOrFalseAnswer::F => String::from("False"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    use std::io::{stdin, stdout, Write};
    let category = "22".to_string();
    let difficulty = "medium".to_string();
    let question_type = "boolean".to_string();

    loop {
        let res = TriviaQuestion::get(&category, &difficulty, &question_type).await;

        match res {
            Ok(res) => {
                println!(
                    "Question: {}",
                    res.results[0].question.replace("&quot;", "\"")
                );

                loop {
                    let mut answer = String::new();
                    let _ = stdout().flush();
                    stdin()
                        .read_line(&mut answer)
                        .expect("Did not enter a correct string");
                    if let Some('\n') = answer.chars().next_back() {
                        answer.pop();
                    }
                    if let Some('\r') = answer.chars().next_back() {
                        answer.pop();
                    }

                    if answer == "Stop" {
                        return Ok(());
                    }

                    let true_or_false: Result<TrueOrFalseAnswer, ()> = answer.try_into();

                    match true_or_false {
                        Ok(ans) => {
                            let true_or_false_string: String = ans.into();
                            if res.results[0].correct_answer == true_or_false_string {
                                println!("Correct!");
                            } else {
                                println!("Incorrect!");
                            }
                            break;
                        }
                        Err(()) => println!("Invalid answer, try again"),
                    }
                }
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }
}
