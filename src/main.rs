extern crate rand;
extern crate regex;
extern crate structopt;

use rand::thread_rng;
use rand::Rng;
use regex::Regex;
use std::error;
use structopt::StructOpt;

use rsg;

#[derive(Debug, StructOpt)]
#[structopt(name = "rsg", about = "Random string generator.")]
struct Opt {
    #[structopt(short, long)]
    len: u32,
    #[structopt(short, long, default_value = "1")]
    number: u32,
    #[structopt(short, long)]
    chars: String,
    #[structopt(long)]
    prefix: Option<String>,
    #[structopt(long)]
    suffix: Option<String>,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let opt = Opt::from_args();
    let chars = rsg::all_chars();

    let mut rng = thread_rng();
    let target = format!(r"^[{}]{{1}}$", opt.chars);
    let re = Regex::new(&target)?;

    let group: Vec<String> = chars.into_iter().filter(|v| re.is_match(&v)).collect();

    for _ in 0..opt.number {
        let mut result = String::new();
        for _ in 0..opt.len {
            let index = rng.gen_range(0..group.len());
            result = result + group.get(index).unwrap();
        }

        if let Some(v) = &opt.prefix {
            result = format!("{}{}", v, result);
        }

        if let Some(v) = &opt.suffix {
            result = format!("{}{}", result, v);
        }

        println!("{}", result);
    }
    Ok(())
}
