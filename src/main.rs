use std::fs::{File, OpenOptions};
use std::borrow::Cow;

const USAGE: &str = "\
usage: medlog <file> <dose_mg>
example: medlog estradiol.log 2
";

fn parse_args() -> Result<(File, u32), Cow<'static, str>> {
    let mut args = std::env::args();
    if args.next().is_none() {
        return Err("missing arguments!".into());
    }
    let file = match args.next() {
        Some(logfile) => logfile,
        None => return Err("missing argument <file>!".into()),
    };
    let file = match OpenOptions::new().append(true).create(true).open(file) {
        Ok(file) => file,
        Err(error) => {
            return Err(format!(
                "can't open or create argument <file>! ({})",
                error
            ).into());
        },
    };
    let dose_mg = match args.next() {
        Some(dose_mg) => dose_mg,
        None => return Err("missing argument <dose_mg>!".into()),
    };
    let dose_mg = match dose_mg.parse::<u32>() {
        Ok(dose_mg) => dose_mg,
        Err(error) => {
            return Err(format!(
                "can't parse argument <dose_mg>! ({})",
                error
            ).into());
        },
    };
    Ok((file, dose_mg))
}

fn main() {
    let (file, dose_mg) = match parse_args() {
        Ok(args) => args,
        Err(error) => {
            println!("{}\nerror: {}", USAGE, error);
            std::process::exit(1);
        },
    };
    println!("*notices your args* owo what's this?: {:?} {}", file, dose_mg);
}

