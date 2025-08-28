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

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    use std::io::{stdin, stdout, Write};
    let category = "22".to_string();
    let difficulty = "medium".to_string();
    let question_type = "boolean".to_string();

    let res = TriviaQuestion::get(&category, &difficulty, &question_type).await?;
    println!("Question: {}", res.results[0].question);

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

    if answer == res.results[0].correct_answer {
        println!("Correct!");
    } else {
        println!("Incorrect!")
    }

    Ok(())
}
