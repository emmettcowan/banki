use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Spend(Payment),
    Show,
}

#[derive(Parser, Debug)]
struct Payment {
    #[arg(short, long)]
    name: String,

    #[arg(short, long)]
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
        Commands::Spend(payment) => {
            println!("{:?}", payment.name);
            conn.execute(
                "INSERT INTO payments (name, amount) VALUES (?1, ?2)",
                (&payment.name, &payment.amount),
            )?;
        }
        Commands::Show => {
            let mut stmt = conn.prepare("SELECT name, amount FROM payments")?;
            let person_iter = stmt.query_map([], |row| {
                Ok(Payment {
                    name: row.get(0)?,
                    amount: row.get(1)?,
                })
            })?;
            for payment in person_iter {
                println!("Found payment {:?}", payment?);
            }
        }
    }
    Ok(())
}
