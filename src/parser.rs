

use clap::Parser;

#[derive(Parser,Debug)]
#[command(version,about,long_about =None)]
pub struct Args{

    #[arg(short,long)]
    pub num: Option<u32>,

    #[arg(short,long)]
    pub list: bool,

}

