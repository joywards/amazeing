use structopt::StructOpt;

#[derive(Debug, PartialEq, Eq, Clone, StructOpt)]
pub struct Args {
    #[structopt(short, long, default_value = "17")]
    pub cell_size: u32,
    #[structopt(short, long)]
    pub debug: bool,
    #[structopt(short, long, default_value = "96")]
    pub invisible_cells_brightness: u8,
}
