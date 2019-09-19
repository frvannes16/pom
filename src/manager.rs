extern crate csv;
extern crate serde_yaml;

use io;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::process;

const POM_DIR: &str = ".pom";
const POM_SETTINGS_FILE: &str = "settings.yaml";
const TASK_RECORDS: &str = "task_history.csv";

#[derive(Serialize, Deserialize)]
struct Settings {
    default_task_time: i64,
    default_break_time: i64,
}

const DEFAULT_SETTINGS: Settings = Settings {
    default_task_time: 25,
    default_break_time: 5,
};

pub fn init_required() -> bool {
    !settings_dir_exists()
}

pub fn init() {
    if init_required() {
        build_pom_dir();
        build_settings_yaml();
    } else {
        println!("Init already completed.")
    }
}

fn build_pom_dir() {
    match fs::DirBuilder::new().create("./.pom") {
        Err(_) => {
            println!("Could not create .pom directory");
            process::exit(1);
        }
        Ok(none) => none,
    };
}

fn build_settings_yaml() {
    let mut settings_yaml = File::create(settings_path()).expect("Could not create settings file.");
    let settings = DEFAULT_SETTINGS;
    let serialized_settings = serde_yaml::to_string(&settings).unwrap();

    match settings_yaml.write_all(serialized_settings.as_bytes()) {
        Ok(none) => none,
        Err(e) => {
            println!("Failed to write pom project settings.yaml file: {}", e);
            process::exit(1);
        }
    };
}

fn settings_dir_exists() -> bool {
    file_exists(POM_DIR)
}

fn settings_path() -> String {
    return format!("{}/{}", POM_DIR, POM_SETTINGS_FILE);
}

fn file_exists(filename: &str) -> bool {
    Path::new(filename).exists()
}

type TaskRecord = (String, i64, i64);

pub fn record_task(
    name: &str,
    task_duration: i64,
    break_duration: i64,
) -> Result<(), Box<dyn Error>> {
    let task_history_file = format!("{}/{}", POM_DIR, TASK_RECORDS);
    let new_record: TaskRecord = (String::from(name), task_duration, break_duration);

    if !file_exists(&task_history_file) {
        init_task_history_file(&task_history_file).expect("Could not create task history csv");
    }
    append_record(&new_record, String::from(task_history_file))
}

fn init_task_history_file(filename: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(filename)?;
    wtr.write_record(&["Task Name", "Task Length", "Break Length"])?;
            wtr.flush()?;
            Ok(())
}

fn append_record(task_record: &TaskRecord, filename: String) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
    .write(true)
    .append(true)
    .create(false)
    .open(filename)
    .unwrap();
    let mut wtr = csv::Writer::from_writer(file);
    wtr.serialize(task_record)?;
    wtr.flush()?;
    Ok(())
}
pub fn print_logs() {
        let task_history_file = format!("{}/{}", POM_DIR, TASK_RECORDS);
    if file_exists(&task_history_file) {
        let mut file = OpenOptions::new().read(true).create(false).open(task_history_file).unwrap();
        io::copy(&mut file, &mut io::stdout()).expect("Could not print logs");
    }
}
