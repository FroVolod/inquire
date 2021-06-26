use inquire::{
    config::Validator, Answer, AskMany, ConfirmOptions, MultiSelectOptions, PasswordOptions,
    QuestionOptions, SelectOptions, Text,
};
use simple_error::bail;

extern crate inquire;

fn main() {
    let fruits = vec![
        "Banana",
        "Apple",
        "Strawberry",
        "Grapes",
        "Lemon",
        "Tangerine",
        "Watermelon",
        "Orange",
        "Pear",
        "Avocado",
        "Pineapple",
    ];

    let languages = vec![
        "C++",
        "Rust",
        "C",
        "Python",
        "Java",
        "TypeScript",
        "JavaScript",
        "Go",
    ];

    let input_validator = |ans: &str| {
        if ans.len() < 5 {
            return Err("Minimum of 5 characters");
        }

        Ok(())
    };

    let pw_validator: Validator = |ans| match ans {
        Answer::Password(val) if val.len() < 8 => bail!("Minimum of 8 characters"),
        Answer::Password(_) => Ok(()),
        _ => panic!("Should not happen"),
    };

    let workplace = Text::new("Where do you work?")
        .with_help_message("Don't worry, this will not be sold to third-party advertisers.")
        .with_validator(input_validator)
        .with_default("Unemployed")
        .prompt()
        .unwrap();

    let answers = vec![
        MultiSelectOptions::new("What are your favorite fruits?", &fruits)
            .unwrap()
            .into_question(),
        ConfirmOptions::new("Do you eat pizza?")
            .with_default(true)
            .into_question(),
        SelectOptions::new("What is your favorite programming language?", &languages)
            .unwrap()
            .into_question(),
        PasswordOptions::new("Password:")
            .with_validator(pw_validator)
            .into_question(),
    ]
    .into_iter()
    .ask()
    .unwrap();

    let eats_pineapple = answers
        .get(1)
        .map(Answer::get_multiple_options)
        .unwrap()
        .into_iter()
        .find(|o| o.index == fruits.len() - 1)
        .is_some();
    let eats_pizza = answers.get(2).map(Answer::get_confirm).unwrap();
    let language = &answers.get(3).map(Answer::get_option).unwrap().value;

    if eats_pineapple && eats_pizza {
        println!("Thanks for your submission.\nWe have reported that a {} developer from {} puts pineapple on pizzas.", language, workplace);
    } else {
        println!("Based on our ML-powered analysis, we were able to conclude absolutely nothing.")
    }
}
