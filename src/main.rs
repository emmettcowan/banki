use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,

    name: String,
}

#[derive(Subcommand)]
enum Commands {
    Spend { amount: Option<u32> },
    Show,
}

#[derive(Debug)]
struct Payment {
    name: String,
    amount: u32,
}

fn main() -> Result<()> {
    let conn = Connection::open("banki.db")?;
    let cli = Cli::parse();

    conn.execute(
        "create table if not exists payments (
             id integer primary key AUTOINCREMENT,
             name TEXT,
             amount INTGERR
         )",
        (),
    )?;

    match &cli.commands {
        Commands::Spend { amount } => {
            let spend_pay = Payment {
                name: cli.name,
                amount: amount.unwrap(),
            };
            println!("{:?}", spend_pay.name);
            conn.execute(
                "INSERT INTO payments (name, amount) VALUES (?1, ?2)",
                (&spend_pay.name, &spend_pay.amount),
            )?;
        }
        Commands::Show => {
            // Fetch rows
            let mut stmt = conn.prepare("SELECT name, amount FROM payments")?;
            let person_iter = stmt.query_map([], |row| {
                Ok(Payment {
                    name: row.get(0)?,
                    amount: row.get(1)?,
                })
            })?;
            // Print results
            for Payment in person_iter {
                println!("Found person {:?}", Payment?);
            }
        }
    }
    Ok(())
}
