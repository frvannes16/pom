extern crate serde_yaml;

use std::fs;
use std::fs::File;
use std::process;
use std::io::prelude::*;

const POM_DIR: &str           = ".pom";
const POM_SETTINGS_FILE: &str = "settings.yaml";

#[derive(Serialize, Deserialize)]
struct Settings {
    default_task_time: i64,
    default_break_time: i64,
}

const DEFAULT_SETTINGS: Settings = Settings {
        default_task_time: 25,
        default_break_time: 5,
    };

pub fn init() {
    if !settings_dir_exists() {
        build_pom_dir();
        build_settings_yaml();
    }
}


fn build_pom_dir() {
    match fs::DirBuilder::new()
        .create("./.pom") {
            Err(_)   => {
                println!("Could not create .pom directory");
                process::exit(1);
            },
            Ok(none) => none,
        };
}


fn build_settings_yaml() {
    let mut settings_yaml = File::create(settings_path())
        .expect("Could not create settings file.");
    let settings = DEFAULT_SETTINGS;
    let serialized_settings = serde_yaml::to_string(&settings).unwrap();

    match settings_yaml.write_all(serialized_settings.as_bytes()) {
        Ok(none) => none,
        Err(e) => {
            println!("Failed to write pom project settings.yaml file: {}", e);
            process::exit(1);},
    };
}

fn settings_dir_exists() -> bool {
    // Checks the current directory to see if the .pom directory exists.
    let mut pom_dir_exists = false;
    let dir_iterator    = fs::read_dir(".").expect("Could not read the current directory");

    for dir_entry in dir_iterator {
        let dir_entry = dir_entry
            .expect("Could not read directory. Directory corrupt");
        println!("{:?}", dir_entry.file_name());

        if dir_entry.file_name().into_string()
            .expect("Could not convert filename to string") == POM_DIR {
            pom_dir_exists = true;
            break;
        }
    }

    return pom_dir_exists;

}

fn settings_path() -> String {
    return format!("{}/{}", POM_DIR, POM_SETTINGS_FILE);
}

