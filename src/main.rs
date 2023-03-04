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
use ansi_term::Colour;
use rand::Rng;
use clap::Parser;

#[derive(Parser)]
#[command(version, about = None)]
struct App {
    /// Determines which type of response to print based on the exit code
    #[arg(short, long, default_value_t = 0)]
    exit_code: i32,
    /// Prints to stdout instead of stderr
    #[arg(short, long)]
    stdout: bool,
}

fn main() {
    let mut rng = rand::thread_rng();

    let app = App::parse();

    // response format
    // {0} - mommy's name
    // {1} - mommy's pronouns
    //  {1.0} - mommy's nominative pronoun
    //  {1.1} - mommy's accusative pronoun
    //  {1.2} - mommy's genitive pronoun
    // {2} - user's name
    let positive_responses = env::var_os("RUST_MOMMY_POSITIVE_RESPONSES").map_or_else(|| vec![
            "{0} is so proud of {1.2} little {2}~ ❤️".to_owned(),
            "good job my little {2}~ ❤️".to_owned(),
            "you're doing great, {2}~ ❤️".to_owned(),
            "{0} thinks {1.2} little {2} is doing a great job~ ❤️".to_owned(),
            "who's {0}'s little {2} ❤️".to_owned(),
        ], |positive_responses| positive_responses.to_str().expect("can't convert OsString to &str").split(';').map(|s| s.to_owned()).collect());

    let negative_responses = env::var_os("RUST_MOMMY_NEGATIVE_RESPONSES").map_or_else(|| vec![
            "don't be so hard on yourself my little {2}~ ❤️".to_owned(),
            "it's okay my little {2}, {0} still loves you~ ❤️".to_owned(),
            "{0}'s sure {1.2} little {2} will get it next time~ ❤️".to_owned(),
            "don't worry {0} still loves you no matter what~ ❤️".to_owned(),
            "even {0} makes mistakes sometimes~ ❤️".to_owned(),
        ], |negative_responses| negative_responses.to_str().expect("can't convert OsString to &str").split(';').map(|s| s.to_owned()).collect());

    // [nom, acc, gen]
    let mommys_pronouns = env::var_os("RUST_MOMMY_PRONOUNS")
        .map_or_else(|| vec![
            "she".to_owned(),
            "her".to_owned(),
            "her".to_owned()
        ],
        |mommys_pronouns| mommys_pronouns.to_str().expect("can't convert OsString to &str").split(';').map(|s| s.to_owned()).collect());

    let mommys_name = env::var_os("RUST_MOMMY_NAME").map_or_else(|| "mommy".to_owned(), |mommys_name| mommys_name.into_string().expect("can't convert OsString to String"));

    let users_names = env::var_os("RUST_MOMMY_USERS_NAMES").map_or_else(|| vec![
        "kitten".to_owned(),
        "baby".to_owned(),
        "sweetie".to_owned(),
        "darling".to_owned(),
        "sweetheart".to_owned(),
        "girl".to_owned(),
    ], |users_names| users_names.to_str().expect("can't convert OsString to &str").split(';').map(|s| s.to_owned()).collect());

    let response_colour = env::var_os("RUST_MOMMY_RESPONSE_COLOUR").map_or((255, 204, 204), |response_colour| {
        let temp = response_colour.to_str().expect("can't convert OsString to &str").split(',').map(|s| s.parse::<u8>().expect("not a number")).collect::<Vec<u8>>();
        (*temp.first().expect("can't get R value"), *temp.get(1).expect("can't get G value"), *temp.get(2).expect("can't get B value"))
    });

    let only_negative = env::var_os("RUST_MOMMY_ONLY_NEGATIVE")
        .map_or(false,|only_positive| 
                only_positive.to_str().expect("can't convert OsString to &str").parse::<bool>().expect("not \"true\" or \"false\"")
                );

    let only_positive = env::var_os("RUST_MOMMY_ONLY_POSITIVE")
        .map_or(false, |only_positive| 
                only_positive.to_str().expect("can't convert OsString to &str").parse::<bool>().expect("not \"true\" or \"false\"")
                );

    if app.exit_code == 0 && only_negative || app.exit_code != 0 && only_positive {
        return;
    }
    
    let rng_index = rng.gen_range(0..positive_responses.len());

    let response = {if app.exit_code == 0 {positive_responses.get(rng_index).expect("can't get positive response")} else {negative_responses.get(rng_index).expect("can't get negative response")}}
        .replace("{0}", &mommys_name)
        .replace("{1.0}", mommys_pronouns.get(0).expect("can't get nominative pronoun"))
        .replace("{1.1}", mommys_pronouns.get(1).expect("can't get accusative pronoun"))
        .replace("{1.2}", mommys_pronouns.get(2).expect("can't get genitive pronoun"))
        .replace("{2}", users_names.get(rng.gen_range(0..users_names.len())).expect("can't get user's name"));

    if app.stdout {
        println!("{}", Colour::RGB(response_colour.0, response_colour.1, response_colour.2).paint(response));
    } else {
        eprintln!("{}", Colour::RGB(response_colour.0, response_colour.1, response_colour.2).paint(response));
    }

    std::process::exit(app.exit_code);
}
