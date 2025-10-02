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
    Remove { id: u32 },
}

#[derive(Parser, Debug)]
struct Payment {
    #[arg(short, long)]
    id: Option<u32>,

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
            conn.execute(
                "INSERT INTO payments (name, amount) VALUES (?1, ?2)",
                (&payment.name, &payment.amount),
            )?;
        }
        Commands::Show => {
            let mut stmt = conn.prepare("SELECT id, name, amount FROM payments")?;
            let person_iter = stmt.query_map([], |row| {
                Ok(Payment {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    amount: row.get(2)?,
                })
            })?;
            for payment in person_iter {
                println!("Found payment {:?}", payment?);
            }
        }
        Commands::Remove { id } => {
            // TODO add remove functionality
            let mut stmt = conn.prepare("DELETE FROM payments WHERE id=(?1)")?;
            stmt.execute([id])?;
        }
    }
    Ok(())
}
