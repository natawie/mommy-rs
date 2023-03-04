/*                                                                           
   Copyright 2023 Natalia Łotocka

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.  
   See the License for the specific language governing permissions and
   limitations under the License.
                                                                             */

use std::env;
use structopt::StructOpt;
use ansi_term::Colour;
use rand::Rng;

#[derive(Debug, StructOpt)]
struct App {
    /// Determines which type of response to print based on the exit code
    #[structopt(short, long, default_value = "0")]
    exit_code: i32,
    /// Prints to stdout instead of stderr
    #[structopt(short, long)]
    stdout: bool,
}

fn main() {
    let mut rng = rand::thread_rng();

    let app = App::from_args();

    // response format
    // {0} - mommy's name
    // {1} - mommy's pronouns
    //  {1.0} - mommy's nominative pronoun
    //  {1.1} - mommy's accusative pronoun
    //  {1.2} - mommy's genitive pronoun
    // {2} - user's name
    let positive_responses = if let Some(positive_responses) = env::var_os("RUST_MOMMY_POSITIVE_RESPONSES") {
        positive_responses.to_str().unwrap().split(';').map(|s| s.to_owned()).collect()
    } else {
        vec![
            "{0} is so proud of {1.2} little {2}~ ❤️".to_owned(),
            "good job my little {2}~ ❤️".to_owned(),
            "you're doing great, {2}~ ❤️".to_owned(),
            "{0} thinks {1.2} little {2} is doing a great job~ ❤️".to_owned(),
            "who's {0}'s little {2} ❤️".to_owned(),
        ]
    };

    let negative_responses = if let Some(negative_responses) = env::var_os("RUST_MOMMY_NEGATIVE_RESPONSES") {
        negative_responses.to_str().unwrap().split(';').map(|s| s.to_owned()).collect()
    } else {
        vec![
            "don't be so hard on yourself my little {2}~ ❤️".to_owned(),
            "it's okay my little {2}, {0} still loves you~ ❤️".to_owned(),
            "{0}'s sure {1.2} little {2} will get it next time~ ❤️".to_owned(),
            "don't worry, {0} still loves you no matter what~ ❤️".to_owned(),
            "even {0} makes mistakes sometimes~ ❤️".to_owned(),
        ]
    };

    // [nom, acc, gen]
    let mommys_pronouns = if let Some(mommys_pronouns) = env::var_os("RUST_MOMMY_PRONOUNS") {
        mommys_pronouns.to_str().unwrap().split(';').map(|s| s.to_owned()).collect::<Vec<String>>()
    } else {
        vec!["she".to_owned(), "her".to_string(), "her".to_string()]
    };

    let mommys_name = if let Some(mommys_name) = env::var_os("RUST_MOMMY_NAME") {
        mommys_name.to_str().unwrap().to_owned()
    } else {
        "mommy".to_owned()
    };

    let users_names = if let Some(users_names) = env::var_os("RUST_MOMMY_USERS_NAMES") {
        users_names.to_str().unwrap().split(';').map(|s| s.to_owned()).collect::<Vec<String>>()
    } else {
        vec![
            "kitten".to_owned(),
            "baby".to_owned(),
            "sweetie".to_owned(),
            "darling".to_owned(),
            "sweetheart".to_owned(),
            "girl".to_owned(),
        ]
    };

    let response_colour = if let Some(response_colour) = env::var_os("RUST_MOMMY_RESPONSE_COLOUR") {
        let temp = response_colour.to_str().unwrap().split(',').map(|s| s.parse::<u8>().unwrap()).collect::<Vec<u8>>();
        (temp[0], temp[1], temp[2])
    } else {
        (255, 204, 204)
    };

    let only_negative = if let Some(only_negative) = env::var_os("RUST_MOMMY_ONLY_NEGATIVE") {
        only_negative.to_str().unwrap().parse::<bool>().unwrap()
    } else {
        false
    };

    let only_positive = if let Some(only_positive) = env::var_os("RUST_MOMMY_ONLY_POSITIVE") {
        only_positive.to_str().unwrap().parse::<bool>().unwrap()
    } else {
        false
    };

    if app.exit_code == 0 && only_negative || app.exit_code != 0 && only_positive {
        return;
    }
    
    let rng_index = rng.gen_range(0..positive_responses.len());

    let mut response = if app.exit_code == 0 {
        positive_responses[rng_index].clone()
    } else {
        negative_responses[rng_index].clone()
    };

    response = response
        .replace("{0}", &mommys_name)
        .replace("{1.0}", &mommys_pronouns[0])
        .replace("{1.1}", &mommys_pronouns[1])
        .replace("{1.2}", &mommys_pronouns[2])
        .replace("{2}", &users_names[rng.gen_range(0..users_names.len())]);

    if app.stdout {
        println!("{}", Colour::RGB(response_colour.0, response_colour.1, response_colour.2).paint(response));
    } else {
        eprintln!("{}", Colour::RGB(response_colour.0, response_colour.1, response_colour.2).paint(response));
    }

    std::process::exit(app.exit_code);
}
