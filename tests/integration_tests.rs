use patient_cli::*;
use std::fs;
use tempfile::TempDir;

const TEST_FILE: &str = "test_patients.json";

// Helper function to clean up before and after each test
fn setup() {
    let _ = fs::remove_file(TEST_FILE);
}

fn teardown() {
    let _ = fs::remove_file(TEST_FILE);
}

#[test]
fn test_add_patient() {
    setup();
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join(TEST_FILE);

    let patient = Patient {
        id: 1,
        name: "John Doe".to_string(),
        diagnosis: "ligma".to_string(),
        medication: "snake oil".to_string(),
        dates: "11/02/25".to_string(),
    };

    add_patient_with_file(&patient, test_file.to_str().unwrap()).expect("Failed to add patient");

    let data = fs::read_to_string(test_file.to_str().unwrap()).expect("Failed to read file");
    let patients: Vec<Patient> = serde_json::from_str(&data).expect("Failed to parse JSON");

    assert!(patients.iter().any(|p| p.id == 1));

    teardown();
}

#[test]
fn test_find_patient_by_id() {
    setup();
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join(TEST_FILE);

    let patient = Patient {
        id: 2,
        name: "Jane Doe".to_string(),
        diagnosis: "no ligma".to_string(),
        medication: "nothing".to_string(),
        dates: "12/02/25".to_string(),
    };

    add_patient_with_file(&patient, test_file.to_str().unwrap()).expect("Failed to add patient");

    let found_patient = find_patients_by_id_with_file(2, test_file.to_str().unwrap())
        .expect("Failed to find patient");
    assert!(found_patient.is_some());
    assert_eq!(found_patient.unwrap().id, 2);

    teardown();
}

#[test]
fn test_delete_patient_by_id() {
    setup();
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join(TEST_FILE);

    let patient = Patient {
        id: 3,
        name: "Bob Doe".to_string(),
        diagnosis: "no ligma".to_string(),
        medication: "nothing".to_string(),
        dates: "12/02/25".to_string(),
    };

    add_patient_with_file(&patient, test_file.to_str().unwrap()).expect("Failed to add patient");

    delete_patients_by_id_with_file(3, test_file.to_str().unwrap())
        .expect("Failed to delete patient");

    let data = fs::read_to_string(test_file.to_str().unwrap()).expect("Failed to read file");
    let patients: Vec<Patient> = serde_json::from_str(&data).expect("Failed to parse JSON");

    assert!(!patients.iter().any(|p| p.id == 3));

    teardown();
}
