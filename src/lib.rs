use std::{fmt::Display, fs::File, io::Write, path::Path, error::Error};
use clap::{Parser, Subcommand}; 
use chrono::{NaiveDate, Datelike, Month}; 
use serde::{Deserialize, Serialize};
use num_traits::cast::FromPrimitive;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands, 
}

/// Subcommands (Add, Delete, Etc.) and their Optional/Mandatory arguments
#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Add {
        #[arg(short = 'k', long)]
        description: String, 
        #[arg(short = 'v', long, default_value_t = 0.0)]
        amount: f32, 
        #[arg(short = 'd', long)]
        date: Option<NaiveDate>, 
    }, 
    Update {
        #[arg(short, long)]
        id: u32, 
        #[arg(short = 'k', long)]
        description: Option<String>,
        #[arg(short = 'v', long)]
        amount: Option<f32>,
        #[arg(short = 'd', long)]
        date: Option<NaiveDate>, 
    },
    Delete {
        #[arg(short, long)]
        id: u32
    },
    List {
        #[arg(short = 'm', long)]
        month: Option<u32>,
    },
    Summary {
        #[arg(short = 'm', long)]
        month: Option<u32>,
    }
}

/// Internal representation of the rows in the CSV file. 
#[derive(Debug, Deserialize, Serialize)]
struct Expense {
    id: u32, 
    amount: f32, 
    description: String,
    date: NaiveDate,
}

impl Expense {
    fn new(id: u32, description: String, amount: f32, date: Option<NaiveDate>) -> Self {
        let date = date.unwrap_or(chrono::Local::now().date_naive()); 
        Expense { id, description, amount, date }
    }
    fn update(&mut self, description: Option<String>, amount: Option<f32>, date: Option<NaiveDate>) {
        if description.is_some() {
            self.description = description.unwrap(); 
        }
        if amount.is_some() {
            self.amount = amount.unwrap();
        }
        if date.is_some() {
            self.date = date.unwrap(); 
        }
    }
}

impl Display for Expense {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let date_str = self.date.format("%Y-%m-%d").to_string();
        write!(f, "{:<3} | {:<10} | {:<10.2} | {}", self.id, date_str, self.amount, self.description)
    }
}

const FILE_PATH: &'static str = "expenses.csv"; 

fn create_db(file_path: &str) -> Result<(), std::io::Error> {
    if !Path::new(file_path).exists() {
        let mut file = File::create(file_path)?;
        // Create a new CSV file with headers
        let _ = file.write_all(b"id;date;description;amount");
    }
    Ok(())
}

/// Reads CSV file (columns separated by ; to avoid issues with different decimal separator (dot or comma)) using Serde for deserialization
fn read_db(file_path: &str) -> Result<Vec<Expense>, csv::Error> {
    let expenses = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .from_path(file_path)?
        .deserialize::<Expense>()
        .filter(|expense| expense.is_ok())
        .map(|expense| expense.unwrap())
        .collect();

    Ok(expenses)
}

/// Writing entries to the CSV file using Serde for serialization
fn write_db(file_path: &str, records: Vec<Expense>) -> Result<(), csv::Error> {
    let mut writer = csv::WriterBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .from_path(file_path)?; 

    for record in records {
        writer.serialize(record)?;
    }
    writer.flush()?;
    Ok(())
}

fn print_db(records: &[Expense]) {
    if records.is_empty() {
        println!("Nothing to list."); 
        return; 
    }
    // Print headers + each entry
    println!("{:<3} | {:<10} | {:<10} | {}", "ID", "Date", "Amount", "Description");
    for entry in records {
        println!("{}", entry);
    }
}

fn filter_records(records: &mut Vec<Expense>, month: Option<u32>) -> Result<(), String> {
    let current_year = chrono::Local::now().year(); 
    if let Some(month) = month {
        if (1..=12).contains(&month) {
            records.retain(|exp| exp.date.month() == month && exp.date.year() == current_year );
        } else {
            return Err("Invalid month (must be a number between 1 and 12)".into());
        }
    }
    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    // Create the CSV file when the user first initializes the app, if one does not exist.
    create_db(FILE_PATH)?;
    // All operations, from reading to writing, require the current list of expenses stored. 
    let mut expenses = read_db(FILE_PATH)?; 
    // Parsing commands 
    let args = Args::parse().cmd;
    match args {
        Commands::Add { description, amount, date } => {
            let id: u32 = if expenses.is_empty() {
                1
            } else {
                expenses.iter().fold(1, |acc, expense| expense.id.max(acc)) + 1 
            }; 
            let new_expense = Expense::new(id, description, amount, date); 
            expenses.push(new_expense); 
            write_db(FILE_PATH, expenses)?;
            println!("Successfully added new expense with ID {id}"); 
        },
        Commands::Update { id, description, amount , date} => {
            if let Some(entry) = expenses.iter_mut().find(|expense| expense.id == id) {
                entry.update(description, amount, date); 
            } else {
                return Err(format!("No entry found with ID = {}", id).into());
            }
            write_db(FILE_PATH, expenses)?;
            println!("Sucessfully updated expense with ID {id}");  
        },
        Commands::Delete { id } => {
            let previous_len = expenses.len();
            expenses.retain(|x| x.id != id);
            // Unequal lengths means the operation was successful 
            if previous_len != expenses.len() { 
                write_db(FILE_PATH, expenses)?; 
                println!("Successully deleted entry with ID {id}"); 
            } else {
                return Err(format!("Expense with id = {} does not exist", id).into());
            }
        },
        Commands::List { month } => {
            // Filter according to month if necessary. 
            filter_records(&mut expenses, month)?;
            print_db(&expenses); 
        },
        Commands::Summary {month} => {
            filter_records(&mut expenses, month)?;
            let total = expenses.iter().fold(0.0, |acc, expense| expense.amount + acc);
            if let Some(month) = month {
                let month_str = Month::from_u32(month).unwrap().name();
                println!("Total expenses for {month_str}: {total}");
            } else {
                println!("Total expenses: {total}");
            }
        }
    }
    Ok(())
}
