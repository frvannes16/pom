use std::fs;
use std::fs::File;
use std::process;

const POM_DIR: &str           = ".pom";
const POM_SETTINGS_FILE: &str = "settings.yaml";
const POM_LOG_FILE: &str      = "pom.log";
const POM_CSV_FILE: &str      = "pom.csv";

pub fn init() {
    if pom_dir_exists() {
        println!("Pom already initiated - .pom directory already exists!");
    } else {
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
    let mut settings_yaml = File::create(settings_path());    
}


fn pom_dir_exists() -> bool {
    // Checks the current directory to see if the .pom directory exists.
    // TODO: Check the parent directories too. How does git do this?
    let mut pom_dir_exists = false;
    let dir_iterator    = fs::read_dir(".").expect("I couldn't read the current directory");

    for dir_entry in dir_iterator {
        let dir_entry = dir_entry
            .expect("The directory is corrupted. There is something I couldn't read");
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

