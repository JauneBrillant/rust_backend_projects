use chrono::NaiveDate;
use clap::{Args, Parser, Subcommand};
use csv::{Reader, Writer, WriterBuilder};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::OpenOptions};

#[derive(Parser)]
#[clap(version = "1.0")]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    New(NewArgs),
    Deposit(DepositArgs),
    Withdraw(WithdrawArgs),
    Import(ImportArgs),
    Report(ReportArgs),
}

#[derive(Args)]
struct NewArgs {
    account_name: String,
}

impl NewArgs {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = Writer::from_path(format!("{}.csv", self.account_name))?;
        writer.write_record(["date", "purpose", "amount"])?;
        writer.flush()?;
        println!("Successfully created '{}' with headers.", self.account_name);
        Ok(())
    }
}

#[derive(Args)]
struct DepositArgs {
    account_name: String,
    date: NaiveDate,
    usage: String,
    amount: u32,
}

impl DepositArgs {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let open_option = OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}.csv", self.account_name))?;

        let mut writer = Writer::from_writer(open_option);
        writer.write_record(&[
            self.date.format("%Y-%m-%d").to_string(),
            self.usage.to_string(),
            self.amount.to_string(),
        ])?;
        writer.flush()?;
        Ok(())
    }
}

#[derive(Args)]
struct WithdrawArgs {
    account_name: String,
    date: NaiveDate,
    usage: String,
    amount: u32,
}

impl WithdrawArgs {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let open_option = OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}.csv", self.account_name))?;

        let mut writer = Writer::from_writer(open_option);
        writer.write_record(&[
            self.date.format("%Y-%m-%d").to_string(),
            self.usage.to_string(),
            format!("-{}", self.amount),
        ])?;
        Ok(())
    }
}

#[derive(Args)]
struct ImportArgs {
    src_file_path: String,
    dst_file_path: String,
}

impl ImportArgs {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let open_option = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.dst_file_path)?;

        let mut writer = WriterBuilder::new()
            .has_headers(false)
            .from_writer(open_option);
        let mut reader = Reader::from_path(&self.src_file_path)?;

        for result in reader.deserialize() {
            let record: Record = result?;
            writer.serialize(record)?;
        }

        Ok(())
    }
}

#[derive(Args)]
struct ReportArgs {
    files: Vec<String>,
}

impl ReportArgs {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut map = HashMap::new();
        for file in &self.files {
            let mut reader = Reader::from_path(file)?;
            for result in reader.records() {
                let record = result?;
                let amount: i32 = record[2].parse()?;
                let date: NaiveDate = record[0].parse()?;
                let sum = map.entry(date.format("%Y-%m").to_string()).or_insert(0);
                *sum += amount
            }
        }
        println!("{:?}", map);
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct Record {
    date: NaiveDate,
    purpose: String,
    amount: i32,
}

fn main() {
    let args = App::parse();
    match args.command {
        Command::New(args) => {
            if let Err(e) = args.run() {
                eprintln!("Error occurred: {}", e);
            }
        }
        Command::Deposit(args) => {
            if let Err(e) = args.run() {
                eprintln!("Error occurred: {}", e);
            }
        }
        Command::Withdraw(args) => {
            if let Err(e) = args.run() {
                eprintln!("Error occurred: {}", e);
            }
        }
        Command::Import(args) => {
            if let Err(e) = args.run() {
                eprintln!("Error occurred: {}", e);
            }
        }
        Command::Report(args) => {
            if let Err(e) = args.run() {
                eprintln!("Error occurred: {}", e);
            }
        }
    }
}
