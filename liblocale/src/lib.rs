use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, prelude::*, Error, ErrorKind},
    path::Path,
    process::Command,
};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Locale {
    pub lang: String,
    pub lc_ctype: String,
    pub lc_numeric: String,
    pub lc_time: String,
    pub lc_collate: String,
    pub lc_monetary: String,
    pub lc_messages: String,
    pub lc_paper: String,
    pub lc_name: String,
    pub lc_address: String,
    pub lc_telephone: String,
    pub lc_measurement: String,
    pub lc_identification: String,
    pub lc_all: String,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LocaleGen {
    pub locales: HashMap<String, LocaleData>,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LocaleData {
    pub country: String,
    pub flag: String,
    pub locale: Locale,
}

pub enum Target {
    Local,
    Global,
}

impl Locale {
    pub fn new() -> Self {
        Self::default()
    }
    // pub fn custom() -> Self {}
    pub fn from_vec_string(data: Vec<String>) -> Locale {
        let mut res: Locale = Locale::default();

        for s in data.clone().iter_mut() {
            let line: Vec<String> = s.split('=').map(|f| f.to_string()).collect();
            match line[0].as_str() {
                "LANG" => {
                    res.lang = line[1].clone().trim_matches('"').to_string();
                }
                "LC_CTYPE" => {
                    res.lc_ctype = line[1].clone().trim_matches('"').to_string();
                }
                "LC_NUMERIC" => {
                    res.lc_numeric = line[1].clone().trim_matches('"').to_string();
                }
                "LC_TIME" => {
                    res.lc_time = line[1].clone().trim_matches('"').to_string();
                }
                "LC_COLLATE" => {
                    res.lc_collate = line[1].clone().trim_matches('"').to_string();
                }
                "LC_MONETARY" => {
                    res.lc_monetary = line[1].clone().trim_matches('"').to_string();
                }
                "LC_MESSAGES" => {
                    res.lc_messages = line[1].clone().trim_matches('"').to_string();
                }
                "LC_PAPER" => {
                    res.lc_paper = line[1].clone().trim_matches('"').to_string();
                }
                "LC_NAME" => {
                    res.lc_name = line[1].clone().trim_matches('"').to_string();
                }
                "LC_ADDRESS" => {
                    res.lc_address = line[1].clone().trim_matches('"').to_string();
                }
                "LC_TELEPHONE" => {
                    res.lc_telephone = line[1].clone().trim_matches('"').to_string();
                }
                "LC_MEASUREMENT" => {
                    res.lc_measurement = line[1].clone().trim_matches('"').to_string();
                }
                "LC_IDENTIFICATION" => {
                    res.lc_identification = line[1].clone().trim_matches('"').to_string();
                }
                "LC_ALL" => {
                    res.lc_all = line[1].clone().trim_matches('"').to_string();
                }
                _ => {}
            }
        }

        res
    }

    pub fn to_locale_string(&self) -> String {
        let locale_string = format!(
            "LANG={lang}\nLC_CTYPE={lc_ctype}\nLC_NUMERIC={lc_numeric}\nLC_TIME={lc_time}\nLC_COLLATE={lc_collate}\nLC_MONETARY={lc_monetary}\nLC_MESSAGES={lc_messages}\nLC_PAPER={lc_paper}\nLC_NAME={lc_name}\nLC_ADDRESS={lc_address}\nLC_TELEPHONE={lc_telephone}\nLC_MEASUREMENT={lc_measurement}\nLC_IDENTIFICATION={lc_identification}\nLC_ALL={lc_all}\n",
            lang = self.lang,
            lc_ctype = self.lc_ctype,
            lc_numeric = self.lc_numeric,
            lc_time = self.lc_time,
            lc_collate = self.lc_collate,
            lc_monetary = self.lc_monetary,
            lc_messages = self.lc_messages,
            lc_paper = self.lc_paper,
            lc_name = self.lc_name,
            lc_address = self.lc_address,
            lc_telephone = self.lc_telephone,
            lc_measurement = self.lc_measurement,
            lc_identification = self.lc_identification,
            lc_all = self.lc_all
        );
        locale_string
    }

    pub fn export(&self, target: Target) -> Result<(), Error> {
        let data = self.to_locale_string();
        match target {
            Target::Local => match env::home_dir() {
                Some(path) => {
                    let home = path.display().to_string();
                    let mut p = Path::new(&home);
                    let np = p.join(".config/locale.conf");

                    println!("{:?}", np.display());
                    match File::create(np) {
                        Ok(mut f) => match f.write_all(&data.as_bytes()) {
                            Ok(_) => Ok(()),
                            Err(e) => Err(e),
                        },
                        Err(e) => Err(e),
                    }
                }
                None => Err(Error::from(ErrorKind::AddrNotAvailable)),
            },
            Target::Global => {
                let p = Path::new("/etc/locale.conf");
                match File::create(p) {
                    Ok(mut f) => match f.write_all(&data.as_bytes()) {
                        Ok(_) => Ok(()),
                        Err(e) => Err(e),
                    },
                    Err(e) => Err(e),
                }
            }
        }
    }

    pub fn resource(&self) -> Result<(), Error> {
        let mut s = String::new();

        // println!("{}", String::from_utf8_lossy(a.stdout.as_ref()));
        let output = Command::new("sh")
            .arg("-c")
            .arg("unset LANG")
            .output()
            .expect("failed to execute process");

        let output1 = Command::new("sh")
            .arg("-c")
            .arg("source /etc/profile.d/locale.sh")
            .output()
            .expect("failed to execute process");

        println!("status: {}", output.status);
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        io::stdout().write_all(&output1.stdout).unwrap();
        io::stderr().write_all(&output1.stderr).unwrap();

        Ok(())
    }
}

impl LocaleGen {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn from(path: &str) -> Result<Self, Error> {
        let file = File::open(path);
        let mut str_data: String = String::new();
        match file {
            Ok(mut f) => {
                f.read_to_string(&mut str_data).unwrap();
                let data = serde_json::from_str(&str_data);
                match data {
                    Ok(d) => Ok(d),
                    Err(e) => {
                        eprintln!("{:?}", e);
                        Err(Error::from(e))
                    }
                }
            }
            Err(e) => {
                eprintln!("{:?}", e);
                Err(e)
            }
        }
    }
    pub fn insert(&mut self, name: String, data: LocaleData) -> Self {
        self.locales.insert(name, data);

        self.clone()
    }
}
