use crossterm::style::Color::Red;
use termimad::{minimad::TextTemplate, MadSkin};

static QUE_TEMPLATE: &str = r#"
-----------
# ${title} 

${question}
-----------
"#;

static ANS_TEMPLATE: &str = r#"
-----------
## ${status} 

${exp}
-----------
## Recommended reading 

${refs}
-----------
"#;

fn main() {
    let q = bnomial_cli::get_question();
    let text_template = TextTemplate::from(QUE_TEMPLATE);
    let mut expander = text_template.expander();
    expander
        .set("title", &q.title)
        .set_lines_md("question", &q.content);

    let default_skin = MadSkin::default();

    default_skin.print_expander(expander);

    let mut wrong_skin = MadSkin::default();
    wrong_skin.set_fg(Red);

    let mut correct_skin = MadSkin::default();
    correct_skin.set_fg(crossterm::style::Color::Green);

    let question = requestty::Question::multi_select("toppings")
        .message("Choices")
        .choices(q.choices.clone())
        .validate(|answer, _| {
            if answer.iter().filter(|&&a| a).count() < 1 {
                Err("You must choose at least one option.".into())
            } else {
                Ok(())
            }
        })
        .transform(|options, _previous_answers, _backend| {
            println!();
            let mut my_choices = ["0", "0", "0", "0"];
            options.iter().for_each(|c| my_choices[c.index] = "1");
            let ans = bnomial_cli::get_answer(my_choices.concat());
            let correct: Vec<bool> = ans.question.answer.chars().map(|c| c == '1').collect();
            for (i, choice) in q.choices.iter().enumerate() {
                if options.iter().any(|x| x.index == i) {
                    let text = format!("❯ {}", choice);

                    if correct[i] {
                        correct_skin.print_text(&text)
                    } else {
                        wrong_skin.print_text(&text)
                    };
                } else {
                    let text = format!("• {}", choice);
                    default_skin.print_text(&text)
                };
            }

            let text_template = TextTemplate::from(ANS_TEMPLATE);
            let mut expander = text_template.expander();
            expander
                // TODO: replace with something like "You almost had it!"
                .set("status", "Explaination")
                .set_lines_md("exp", &ans.question.explanation)
                .set_lines_md("refs", &ans.question.references);

            default_skin.print_expander(expander);
            Ok(())
        })
        .build();

    requestty::prompt_one(question).expect("Expected to get answer successfully");
}
