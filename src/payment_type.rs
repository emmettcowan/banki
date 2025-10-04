use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Payment {
    #[arg(short, long)]
    pub id: Option<u32>,

    #[arg(short, long)]
    pub name: String,

    #[arg(short, long)]
    pub amount: u32,
}
