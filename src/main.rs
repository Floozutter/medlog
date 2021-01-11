const USAGE: &str = "\
usage: medlog <logfile> <dose_mg>
example: medlog estradiol.log 2
";

fn parse_args() -> Result<(String, u32), String> {
    let mut args = std::env::args();
    args.next();
    let logfile = match args.next() {
        Some(logfile) => logfile,
        None => return Err("missing argument <logfile>!".to_string()),
    };
    let dose_mg = match args.next() {
        Some(dose_mg) => dose_mg,
        None => return Err("missing argument <dose_mg>!".to_string()),
    };
    let dose_mg = match dose_mg.parse::<u32>() {
        Ok(dose_mg) => dose_mg,
        Err(error) => {
            return Err(format!(
                "can't parse argument <dose_mg>! ({})",
                error.to_string()
            ))
        },
    };
    Ok((logfile, dose_mg))
}

fn main() {
    let (logfile, dose_mg) = match parse_args() {
        Ok(args) => args,
        Err(error) => {
            println!("{}\nerror: {}", USAGE, error);
            std::process::exit(1);
        },
    };
    println!("*notices your args* owo what's this?: {} {}", logfile, dose_mg);
}

