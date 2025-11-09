//use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::fs::read_to_string;
use std::io;
use std::io::BufRead;
use std::io::Read;
use std::io::Write;

const PATIENTS_FILE: &str = "patients.json";
macro_rules! update_field {
    ($patient:expr, $field:expr, $value:expr, $type:ty) => {
        match $field {
            "id" => $patient.id = $value.parse().expect("Invalid id"),
            "name" => $patient.name = $value.to_string(),
            "medication" => $patient.medication = $value.to_string(),
            "diagnosis" => $patient.diagnosis = $value.to_string(),
            "dates" => $patient.dates = $value.to_string(),
            _ => {}
        }
        println!("Updating ID {}, {}", $patient.id, $patient.name);
    };
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Patient {
    pub id: i32,
    pub name: String,
    pub diagnosis: String,
    pub medication: String,
    pub dates: String,
}
impl Patient {
    pub fn patient_file_management(&self) -> std::io::Result<()> {
        let mut patients: Vec<Patient> = if fs::metadata(PATIENTS_FILE).is_ok() {
            let mut file = fs::File::open(PATIENTS_FILE)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            serde_json::from_str(&contents).unwrap_or_else(|_| vec![])
        } else {
            vec![]
        };
        match add_patient(patients, self) {
            Ok(_) => {}
            Err(e) => {
                println!("{e}")
            }
        }

        println!("saving...");
        Ok(())
    }
}
//Self explanitory...read serde documentation for more info
pub fn add_patient(
    mut patients: Vec<Patient>,
    patient: &Patient,
) -> Result<(), Box<dyn std::error::Error>> {
    if patients.iter().any(|p| p.id == patient.id) {
        println!("Looking for ID {} ...", patient.id);
        return Err("Patient with this ID already exists".into());
    }

    patients.push(patient.clone());
    save_patients(&patients)?;

    Ok(())
}

pub fn list_patients() {
    match load_patients() {
        Ok(p) => {
            let patients = p;
            for patient in patients {
                println!("Listing {:?}", patient);
            }
        }
        Err(e) => {
            println!("We have a problen: {}", e);
        }
    };
}

pub fn find_patients_by_id(id: i32) -> Result<Option<Patient>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(PATIENTS_FILE)?;
    let patients: Vec<Patient> = serde_json::from_str(&data)?;

    Ok(patients.into_iter().find(|p| p.id == id))
}

pub fn delete_patients_by_id(id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string(PATIENTS_FILE)?;
    let patients: Vec<Patient> = serde_json::from_str(&data)?;
    let filtered: Vec<Patient> = patients.into_iter().filter(|p| p.id != id).collect();

    fs::write(PATIENTS_FILE, serde_json::to_string_pretty(&filtered)?)?;
    Ok(())
}

pub fn input_reader_cli(id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string(PATIENTS_FILE)?;
    let mut patients: Vec<Patient> = serde_json::from_str(&data)?;

    let stdout = io::stdout();
    let mut user_input: String = String::new();
    let mut counter: i32 = 0;

    let mut write_buffer = io::BufWriter::new(stdout.lock());
    writeln!(
        write_buffer,
        "\n--------------------Update--------------------"
    )?;
    //TODO: Write clearer instructions
    writeln!(
        write_buffer,
        "Type something like --[entry] to update the said entry ... press # on a new line to end;\n ,,,"
    )?;
    write_buffer.flush()?;

    let reader = io::stdin().lock();
    // Getting user input from cmd
    for line in reader.lines() {
        counter += 1;
        if let Ok(line) = line {
            if line == "#" {
                break;
            }
            user_input.push_str(&line);
            user_input.push('\n');
        }
        if counter == 5 {
            break; // Counter dictates how many lines are possible so that users wont be stuck in an input loop.
        }
    }

    let parsed_args = parse_arguments(&user_input); //This parses user input into args as a hash
    for patient in &mut patients {
        if patient.id == id {
            for key in parsed_args.keys() {
                let value = parsed_args
                    .get(key)
                    .unwrap_or(&"Empty".to_string())
                    .to_string();
                update_field!(patient, key.as_str(), value, i32);
            }
        }
    }
    write_buffer.flush()?;
    save_patients(&patients)?;

    Ok(())
}

fn load_patients() -> Result<Vec<Patient>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(PATIENTS_FILE)?;
    Ok(serde_json::from_str(&data).unwrap_or_else(|_| vec![]))
}

fn save_patients(patients: &Vec<Patient>) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(PATIENTS_FILE, to_string_pretty(patients)?)?;
    Ok(())
}

//To complicated for me, shouldve used clap... welp too late. Code may or may not break. Ill fix this later.
fn parse_arguments(input: &str) -> HashMap<String, String> {
    let mut args = HashMap::new();
    let mut tokens = input.split_whitespace();

    while let Some(key) = tokens.next() {
        if key.starts_with("--") {
            let key = key.trim_start_matches("--").to_string();
            if let Some(value) = tokens.next() {
                args.insert(key, value.to_string());
            }
        }
    }

    args
}
