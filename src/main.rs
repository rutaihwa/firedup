use clap::{App, Arg};
use std::process::Command;

fn main() {
    // Creating an application
    let matches = App::new("Firedup - A wrapper for firefox")
        .version("0.1.0")
        .author("Rweye")
        .arg(
            Arg::with_name("skin")
                .short("s")
                .takes_value(true)
                .help("Which skin are you taking?"),
        )
        .get_matches();

    // Skin for profile
    let skin = matches.value_of("skin");

    // firefox
    let firefox = Command::new("/usr/bin/firefox");

    // Firing firefox
    fn fire(skin: String, mut fox: Command) {
        if skin.is_empty() {
            fox.status().expect("Couldn't fire");
        } else {
            fox.arg("-p").arg(skin).status().expect("Couldn't fire");
        }
    }
    // Checking the skin
    match skin {
        None => {
            fire("".to_string(), firefox);
        }
        Some(s) => match s.parse::<String>() {
            Ok(n) => {
                fire(n, firefox);
            }
            Err(_) => {
                fire("".to_string(), firefox);
            }
        },
    }
}
