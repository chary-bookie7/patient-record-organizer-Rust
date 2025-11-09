use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Read;

use crate::patient_fn::delete_patients_by_id;
use crate::patient_fn::find_patients_by_id;
use crate::patient_fn::input_reader_cli;
use crate::patient_fn::list_patients;

mod patient_fn;

//TODO:
//~Sometimes ID's can clash. Find a way to improve upon that.
//~Structure data using FHIR (Fast Healthcare Interoperability Resources) basics.

#[derive(Parser)]
#[command(name = "Patient Record Organizer CLI (in Rust)")]
#[command(about = "Organise patient records... in rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    Add {
        #[arg(long, help = "Add a --id of the patient")]
        id: i32,
        #[arg(long, help = "Add a --name of the patient")]
        name: String,
        #[arg(long, help = "Add the --diagnostics of the patient")]
        diagnosis: String,
        #[arg(long, help = "What --medicine should they have")]
        medication: String,
        #[arg(long)]
        dates: String,
    },
    View {
        #[arg(long)]
        id: i32,
    },
    Update {
        #[arg(long)]
        id: i32,
    },

    Delete {
        #[arg(long)]
        id: i32,
    },
    List {
        //Ill add exeptions later
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add {
            id,
            name,
            diagnosis,
            medication,
            dates,
        } => {
            let patient = patient_fn::Patient {
                id: id.clone(),
                name: name.clone(),
                diagnosis: diagnosis.clone(),
                medication: medication.clone(),
                dates: dates.clone(),
            };
            patient.patient_file_management().unwrap();
        }
        Commands::View { id } => match find_patients_by_id(*id) {
            Ok(Some(p)) => println!("Found: {:?}", p),
            Ok(None) => println!("Patient not found."),
            Err(e) => println!("Error: {}", e),
        },
        Commands::Update { id } => {
            let _ = input_reader_cli(*id);
        }
        Commands::Delete { id } => match delete_patients_by_id(*id) {
            Ok(_) => println!("Patient deleted."),
            Err(e) => println!("Error: {}", e),
        },
        Commands::List {} => list_patients(),
    }
}
