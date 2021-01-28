use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::process::Command;

impl LocaleGen {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, name: String, data: LocaleData) -> Self {
        self.locales.insert(name, data);

        self.clone()
    }
}

fn main() -> Result<(), Error> {
    let mut file = File::open("locale.gen")?;
    let mut str_data: String = String::new();
    let mut lcs: LocaleGen = LocaleGen::new();

    file.read_to_string(&mut str_data)?;

    let mut vec_data: Vec<String> = str_data
        .split("# ")
        .filter(|l| !l.is_empty())
        .map(|f| f.to_string())
        .collect();
    vec_data.iter_mut().for_each(|l| {
        l.pop();
        if l.contains(" ") {
            let a: Vec<String> = l.split(" ").map(|f| f.to_string()).collect();
            *l = a[0].to_owned();
        }
    });

    for lc in vec_data.iter() {
        let country_cmd = format!("LANG={} locale country_name", &lc);
        let locale_cmd = format!("LANG={} locale", &lc);
        let country_output = Command::new("sh").arg("-c").arg(country_cmd).output();
        let locale_output = Command::new("sh").arg("-c").arg(locale_cmd).output();

        let mut locale: LocaleData = LocaleData::default();

        match country_output {
            Ok(d) => {
                locale.country = String::from_utf8_lossy(&d.stdout)
                    .get(0..)
                    .unwrap()
                    .trim_matches('\n')
                    .to_string();
            }
            Err(e) => {
                eprint!("{:#?}", e);
            }
        }

        match locale_output {
            Ok(d) => {
                // println!("{}", );
                let data: String = String::from_utf8_lossy(&d.stdout)
                    .get(0..)
                    .unwrap()
                    .to_string();
                let v_data: Vec<String> = data.lines().into_iter().map(|s| s.to_string()).collect();
                let dt = conf_parser(v_data);
                // println!("{:#?}", dt);
                locale.locale = dt;
            }
            Err(e) => {
                eprint!("{:#?}", e);
            }
        }

        lcs.insert(lc.to_string(), locale);
    }

    // println!("{}", serde_json::to_string_pretty(&lcs).unwrap());
    let json_data = serde_json::to_string_pretty(&lcs).unwrap();

    match File::create("locales.json") {
        Ok(mut f) => f.write_all(json_data.as_bytes()).unwrap(),
        Err(e) => eprint!("{:#?}", e),
    }

    Ok(())
}
