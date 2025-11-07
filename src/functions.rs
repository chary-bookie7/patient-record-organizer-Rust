use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Clone)]
pub struct Patient {
    id:i32
    name: String,
    diagnosis: String,
    medication: String,
    dates:String,
}
impl Patient{
    pub fn patient_file_management(patient: &Patient) -> std::io::Result<()> {
        let mut patients: Vec<Patient> = if fs::metadata("patients.json").is_ok() {
            let mut file = fs::File::open("patients.json")?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            serde_json::from_str(&contents).unwrap_or_else(|_| vec![])
        } else {
            vec![]
        };

        patients.push(patient.clone());
        let json_data = serde_json::to_string_pretty(&patients)?;
        fs::write("patients.json", json_data)?;
        Ok(())
    }

    pub fn retreave_patient_json(name: String)->Patient {
        return Patient{id,name,diagnostics,medication,dates}
    }
}
