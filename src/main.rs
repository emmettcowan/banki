use anyhow::Result;
use clap::{Parser, Subcommand};
use rusqlite::Connection;
mod payment_type;
mod server;
mod sql;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Spend(payment_type::Payment),
    Show,
    Remove { id: u32 },
    Serve,
}

fn main() -> Result<()> {
    let mut conn = Connection::open("banki.db")?;
    let cli = Cli::parse();

    sql::create(&mut conn)?;

    match &cli.commands {
        Commands::Spend(payment) => {
            sql::insert_payment(&mut conn, payment)?;
        }
        Commands::Show => {
            sql::list_payment(&mut conn)?;
        }
        Commands::Remove { id } => {
            sql::remove_payment(&mut conn, id)?;
        }
        Commands::Serve => server::run()?,
    }
    Ok(())
}
