use clap::Parser;

#[derive(Parser, Debug)]
pub struct Payment {
    // TODO find better way do id so its not optional key
    #[arg(short, long)]
    pub id: Option<u32>,

    #[arg(short, long)]
    pub name: String,

    #[arg(short, long)]
    pub amount: u32,
}
