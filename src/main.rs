use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Read;

mod functions;

//TODO, Add a feature to get info from file to command line
//Model data using FHIR (Fast Healthcare Interoperability Resources) basics.

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
        #[arg(long)]
        id:i32,
        #[arg(long, help = "Add a --name of the patient")]
        name: String,
        #[arg(long, help = "Add the --diagnostics of the patient")]
        diagnostics: String,
        #[arg(long, help = "What --medicine should they have")]
        medication: String,
        #[arg(long)]
        dates:String,
    },
    View {
        #[arg(long)]
        id:i32
    },
    Update {
        #[arg(long)]
        id:i32,
    },

    Delete {
        #[arg(long)]
        id:i32,
    },
}

#[derive(Serialize, Deserialize, Clone)]
struct Patient {
    id:i32
    name: String,
    diagnosis: String,
    medication: String,
    dates:String,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add {
            name,
            diagnostics,
            medicine,
        } => {
            let patient = Patient {
                name: name.clone(),
                diagnosis: diagnostics.clone(),
                medication: medicine.clone(),
            };
            patient_file_management(&patient).unwrap();
        },
        Commands::View {id}=>{},
        Commands::Update {id}=>{
            let patient = retreave_patient_json();
            println!("Update: {} -> Options: ",patient.name)
            let options = String::new();
        },
        Commands::Delete {id}=>{},

    }
}
