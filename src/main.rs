use std::{collections::BTreeMap, fs, io};
use rand::random;
use toml;
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Choice {
    name: String,
    poss: f64,
    subchoices: Option<Vec<Choice>>,
}

#[derive(Deserialize)]
struct Config {
    env: String,
    locale: String,
    name: String,
    choices: Vec<Choice>,
}

fn decide_cli(locale: &BTreeMap<String, String>, choices: &Vec<Choice>) {
    for (index, choice) in choices.iter().enumerate() {
        println!("{}[{}]: {}", locale["item_choice"], index, choice.name);
        println!("{}: {}", locale["item_possibility"], choice.poss);
    }

    let mut is_predecide = false;

    println!("{}(y/N)", locale["prompt_is_predecide"]);

    let mut prompt = String::new();
    io::stdin().read_line(&mut prompt)
        .expect(locale["err_unable_to_read"].as_ref());
    prompt.make_ascii_lowercase();
    if prompt.trim() == "y" {
        is_predecide = true;
    }

    let mut choice = choices.get(0).unwrap();
    if is_predecide {
        let mut prompt = String::new();
        println!("{}", locale["prompt_choice_number"]);
        io::stdin().read_line(&mut prompt)
            .expect(locale["err_unable_to_read"].as_ref());
        let choice_index: usize = match prompt.trim().parse() {
            Ok(choice_index) => choice_index,
            Err(_) => 0,
        };
        choice = choices.get(choice_index).unwrap();
        println!("{}: {}\n", locale["msg_choice"], choice.name);
    } else {
        let dice: f64 = random();
        let mut tot: f64 = 0.0;
        for choice_el in choices.iter() {
            tot += choice_el.poss;
            if tot > dice {
                choice = choice_el;
                break;
            }
        }
        println!("{}: {}\n", locale["msg_god_choice"], choice.name);
    }

    if ! choice.subchoices.is_none() {
        decide_cli(locale, choice.subchoices.as_ref().unwrap());
    }
}

fn main() {
    let config_str = fs::read_to_string(
        "config.toml"
        )
        .expect("Unable to read config file");
    let config: Config = toml::from_str(&config_str)
        .expect("Unable to parse config");
    let locale_str = fs::read_to_string(
        format!("locale.{}.toml", config.locale)
        )
        .expect("Unable to read locale file");
    let locale: BTreeMap<String, String> = toml::from_str(&locale_str)
        .expect("Unable to parse locale file");

    match config.env.as_str() {
        "api" => {
            // use api to fetch and randomise
        },
        "ws" => {
            // warm up web socket
        },
        "cli" => {
            // run directly
            println!("{} {}\n", locale["begin"], config.name);
            decide_cli(&locale, config.choices.as_ref());
        },
        _ => {
            panic!("No environment")
        }
    };
}
