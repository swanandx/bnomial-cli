use serde::Deserialize;

const QUESTION_API: &str =
    "https://eyodmpf7zi.execute-api.us-east-1.amazonaws.com/prod/public/question";

#[derive(Deserialize, Debug)]
pub struct Question {
    pub date: String,
    pub title: String,
    pub content: String,
    pub choices: Vec<String>,
    pub notebook: Option<String>,
    pub answers: u32,
    pub performance: f32,
    #[serde(default)]
    pub answer: String,
    #[serde(default)]
    pub explanation: String,
    #[serde(default)]
    pub references: String,
}

#[derive(Deserialize, Debug)]
struct Today {
    question: Question,
}

#[derive(Deserialize, Debug)]
pub struct Answer {
    pub answer: String,
    pub date: String,
    pub choices: u8,
    pub correct: u8,
}

#[derive(Deserialize, Debug)]
struct Resp {
    today: Today,
}

#[derive(Deserialize, Debug)]
pub struct Response {
    pub question: Question,
    pub answer: Answer,
}

pub fn get_question() -> Question {
    let resp: Resp = reqwest::blocking::get(QUESTION_API)
        .expect("Expected to get question")
        .json()
        .unwrap();
    // NOTE: For local testing, comment code above and use _SAMPLE_GET_RESP instead
    // let re: Resp = serde_json::from_str(_SAMPLE_GET_RESP).unwrap();
    resp.today.question
}

// POST - {"answer":"1010","tracking":true}
pub fn get_answer(choices: String) -> Response {
    let mut map = ::std::collections::HashMap::new();
    map.insert("answer", choices);

    let client = reqwest::blocking::Client::new();
    let resp: Response = client
        .post(QUESTION_API)
        .json(&map)
        .send()
        .unwrap()
        .json()
        .unwrap();
    // NOTE: For local testing, comment code above and use _SAMPLE_POST_RESP instead
    // let res: Response = serde_json::from_str(_SAMPLE_POST_RESP).unwrap();
    resp
}

const _SAMPLE_GET_RESP: &str = r#"
{
    "today": {
        "question": {
            "date": "20220817",
            "title": "A classification model",
            "content": "Natalie has been a software developer for quite some time. Although her job keeps her motivated, she wants to take her career to the next level.\n\nA critical step she wants to take is introducing machine learning into her work. She started learning some of the fundamentals and is now ready to apply what she's learned.\n\nShe researched one of her company's problems and learned she needed to build a supervised learning classification model. She had enough labeled data, so it seemed like a good fit.\n\n**Based on this, which of the following better describes what Natalie needs to accomplish?**",
            "choices": [
                "She needs to train a model that returns a numerical prediction for each sample of data.",
                "She needs to train a model that clusters the data into different groups based on their characteristics.",
                "She needs to train a model to predict the class of every sample of data out of a predefined list of classes.",
                "She needs to train a model that returns the optimal policy that maximizes the potential outcomes of her problem."
            ],
            "notebook": null,
            "answers": 1206,
            "performance": 0.9
        }
    }
}
"#;

const _SAMPLE_POST_RESP: &str = r#"
{
    "question": {
        "date": "20220817",
        "title": "A classification model",
        "content": "Natalie has been a software developer for quite some time. Although her job keeps her motivated, she wants to take her career to the next level.\n\nA critical step she wants to take is introducing machine learning into her work. She started learning some of the fundamentals and is now ready to apply what she's learned.\n\nShe researched one of her company's problems and learned she needed to build a supervised learning classification model. She had enough labeled data, so it seemed like a good fit.\n\n**Based on this, which of the following better describes what Natalie needs to accomplish?**",
        "choices": [
            "She needs to train a model that returns a numerical prediction for each sample of data.",
            "She needs to train a model that clusters the data into different groups based on their characteristics.",
            "She needs to train a model to predict the class of every sample of data out of a predefined list of classes.",
            "She needs to train a model that returns the optimal policy that maximizes the potential outcomes of her problem."
        ],
        "notebook": null,
        "answers": 1183,
        "performance": 0.9,
        "answer": "0010",
        "explanation": "Natalie's problem requires her to predict the class of every sample of data out of a predefined list of classes. That's the goal of machine learning classification models.\n\nThe first choice refers to a [regression](https://en.wikipedia.org/wiki/Regression_analysis) model. Here we want the model to output a single, continuous value. For example, imagine we want to return the predicted price of a house or the predicted highest temperature for the weekend. \n\nThe second choice refers to a [clustering](https://en.wikipedia.org/wiki/Cluster_analysis) model. These unsupervised learning techniques are helpful when we don't have labels for our data and want the algorithm to group every sample into dynamically generated groups.\n\nThe fourth choice is a loose description of a [reinforcement learning](https://en.wikipedia.org/wiki/Reinforcement_learning) approach, where we want an agent to learn the optimal policy that maximizes a reward function.",
        "references": "* [\"4 Types of Classification Tasks in Machine Learning\"](https://machinelearningmastery.com/types-of-classification-in-machine-learning/) is an excellent introduction to classification models in machine learning.\n* For an introduction to classification models, check [\"Classification in Machine Learning: What it is and Classification Models\"](https://www.simplilearn.com/tutorials/machine-learning-tutorial/classification-in-machine-learning)"
    },
    "answer": {
        "answer": "1010",
        "date": "20220817",
        "choices": 4,
        "correct": 3
    }
}
"#;
