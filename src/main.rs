use std::io;
use rand::Rng;

mod types;
mod verben;

use verben::get_verben;

fn main() {
    let verben_list = get_verben();
    let mut rng = rand::thread_rng();

    let verben = &verben_list[rng.gen_range(0, verben_list.len())];

    let person = ["ich", "du", "er, sie, es", "wir", "ihr", "Sie"];

    let mut input = String::new();
    let mut conjugation_ite;

    for verb in verben.iter() {
        conjugation_ite = 0;
        println!(" --- {} ({:?} {:?}) ---", verb.name, verb.verb_type, verb.zeit_type);
        loop {
            println!("{}:", person[conjugation_ite]);
            match io::stdin().read_line(&mut input) {
                Ok(_n) => {
                    match input.trim() == verb.conjugations[conjugation_ite] {
                        true => {
                            conjugation_ite += 1;
                        }
                        _ => {
                            println!("{} != {}", input.trim(), verb.conjugations[conjugation_ite]);
                        }
                    }

                    input = String::new();

                    if conjugation_ite == 6 {
                        break;
                    }
                }
                Err(error) => {
                    println!("error: {}", error);
                }
            }
        }
    }
}
