use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rowops", about = "easily manipulated matrices visually")]

pub struct Opt {
    #[structopt(short = "c", default_value = "4")]
    cols: u8,

    #[structopt(short = "r", default_value = "3")]
    rows: u8,
}

impl Opt {
    pub fn create() -> Self {
        Opt::from_args()
    }
}
