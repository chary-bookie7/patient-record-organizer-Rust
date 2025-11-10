use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::collections::HashMap;
use std::fs;
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
        match add_patient(self) {
            Ok(_) => {}
            Err(e) => {
                println!("{e}")
            }
        }

        println!("saving...");
        Ok(())
    }
}

pub fn add_patient(patient: &Patient) -> Result<(), Box<dyn std::error::Error>> {
    add_patient_with_file(patient, PATIENTS_FILE)
}

pub fn add_patient_with_file(
    patient: &Patient,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut patients: Vec<Patient> = if fs::metadata(file_path).is_ok() {
        let mut file = fs::File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        serde_json::from_str(&contents).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    };

    if patients.iter().any(|p| p.id == patient.id) {
        println!("Looking for ID {} ...", patient.id);
        return Err("Patient with this ID already exists".into());
    }

    patients.push(patient.clone());
    save_patients_to_file(&patients, file_path)?;

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
            println!("We have a problem: {}", e);
        }
    };
}

pub fn find_patients_by_id(id: i32) -> Result<Option<Patient>, Box<dyn std::error::Error>> {
    find_patients_by_id_with_file(id, PATIENTS_FILE)
}

pub fn find_patients_by_id_with_file(
    id: i32,
    file_path: &str,
) -> Result<Option<Patient>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(file_path)?;
    let patients: Vec<Patient> = serde_json::from_str(&data)?;

    Ok(patients.into_iter().find(|p| p.id == id))
}

pub fn delete_patients_by_id(id: i32) -> Result<(), Box<dyn std::error::Error>> {
    delete_patients_by_id_with_file(id, PATIENTS_FILE)
}

pub fn delete_patients_by_id_with_file(
    id: i32,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string(file_path)?;
    let patients: Vec<Patient> = serde_json::from_str(&data)?;
    let filtered: Vec<Patient> = patients.into_iter().filter(|p| p.id != id).collect();

    fs::write(file_path, serde_json::to_string_pretty(&filtered)?)?;
    Ok(())
}

pub fn input_reader_cli(id: i32) -> Result<(), Box<dyn std::error::Error>> {
    input_reader_cli_with_file(id, PATIENTS_FILE)
}

pub fn input_reader_cli_with_file(
    id: i32,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string(file_path)?;
    let mut patients: Vec<Patient> = serde_json::from_str(&data)?;

    let stdout = io::stdout();
    let mut user_input: String = String::new();
    let mut counter: i32 = 0;

    let mut write_buffer = io::BufWriter::new(stdout.lock());
    writeln!(
        write_buffer,
        "\n--------------------Update--------------------"
    )?;
    writeln!(
        write_buffer,
        "Type something like --[entry] to update the said entry ... press # on a new line to end;\n ,,,"
    )?;
    write_buffer.flush()?;

    let reader = io::stdin().lock();
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
            break;
        }
    }

    let parsed_args = parse_arguments(&user_input);
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
    save_patients_to_file(&patients, file_path)?;

    Ok(())
}

fn load_patients() -> Result<Vec<Patient>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(PATIENTS_FILE)?;
    Ok(serde_json::from_str(&data).unwrap_or_else(|_| vec![]))
}

fn save_patients(patients: &Vec<Patient>) -> Result<(), Box<dyn std::error::Error>> {
    save_patients_to_file(patients, PATIENTS_FILE)
}

fn save_patients_to_file(
    patients: &Vec<Patient>,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(file_path, to_string_pretty(patients)?)?;
    Ok(())
}

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
