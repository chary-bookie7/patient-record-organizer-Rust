//use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Read;

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
        let mut patients: Vec<Patient> = if fs::metadata("patients.json").is_ok() {
            let mut file = fs::File::open("patients.json")?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            serde_json::from_str(&contents).unwrap_or_else(|_| vec![])
        } else {
            vec![]
        };

        patients.push(self.clone());
        let json_data = serde_json::to_string_pretty(&patients)?;
        fs::write("patients.json", json_data)?;
        Ok(())
    }
}
pub fn find_patients_by_id(id: i32) -> Result<Option<Patient>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string("patients.json")?;
    let patients: Vec<Patient> = serde_json::from_str(&data)?;
    Ok(patients.into_iter().find(|p| p.id == id))
}
pub fn delete_patients_by_id(id: i32) -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string("patients.json")?;
    let patients: Vec<Patient> = serde_json::from_str(&data)?;
    let filtered: Vec<Patient> = patients.into_iter().filter(|p| p.id != id).collect();
    fs::write("patients.json", serde_json::to_string_pretty(&filtered)?)?;
    Ok(())
}
