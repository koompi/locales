use clap::{App, Arg};
use liblocale::*;
use std::{fs::File, io::prelude::*, io::Error};
fn main() -> Result<(), Error> {
    let locales: LocaleGen = LocaleGen::from("locales.json").unwrap();

    let matches = App::new("PIDE Locale Manager")
        .version("1.0")
        .author("Brilliant PHAL. <brilliant@koompi.com>")
        .about("Manage system locale settings")
        .arg(
            Arg::new("set")
                .short('s')
                .long("set")
                .value_name("Locale name")
                .about("set new locale")
                .takes_value(true),
        )
        .arg(
            Arg::new("global")
                .short('g')
                .about("Local or Global")
                .conflicts_with("local"),
        )
        .arg(
            Arg::new("local")
                .short('l')
                .about("Local or Global")
                .conflicts_with("global"),
        )
        .get_matches();

    if let Some(i) = matches.value_of("set") {
        if let g = matches.occurrences_of("global") {
            if g == 0 {
                println!("generating local config");
                match locales.locales.get(i) {
                    Some(l) => {
                        // println!("{:#?}", l);
                        let target_local = l.locale.export(Target::Local);
                        match target_local {
                            Ok(_) => {
                                println!("Created.");
                                l.locale.resource();
                            }
                            Err(e) => println!("{}", e),
                        }
                    }
                    None => {
                        println!("{} is not a valid locale", i)
                    }
                }
            } else {
                println!("generating global config");
                match locales.locales.get(i) {
                    Some(l) => {
                        let target_local = l.locale.export(Target::Global);
                        match target_local {
                            Ok(_) => {
                                println!("Created.");
                                l.locale.resource();
                            }
                            Err(e) => println!("{}", e),
                        }
                    }
                    None => {
                        println!("{} is not a valid locale", i)
                    }
                }
            }
        }
    }

    Ok(())
}
