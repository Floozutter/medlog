const USAGE: &str = "\
usage: medlog <log_file> <dose>
example: medlog estradiol-mg.log 2
";

fn parse_args() -> Result<(std::fs::File, u32), std::borrow::Cow<'static, str>> {
    let mut args = std::env::args();
    if args.next().is_none() {
        return Err("missing arguments!".into());
    }
    let log_file = match args.next() {
        Some(log_file) => log_file,
        None => return Err("missing argument <log_file>!".into()),
    };
    let log_file = match std::fs::OpenOptions::new().append(true).create(true).open(&log_file) {
        Ok(log_file) => log_file,
        Err(error) => return Err(format!(
            "can't open or create argument <log_file>: `{}` ({})!",
            log_file, error
        ).into()),
    };
    let dose = match args.next() {
        Some(dose) => dose,
        None => return Err("missing argument <dose>!".into()),
    };
    let dose = match dose.parse::<u32>() {
        Ok(dose) => dose,
        Err(error) => return Err(format!(
            "can't parse argument <dose>: `{}` ({})!",
            dose, error
        ).into()),
    };
    Ok((log_file, dose))
}

fn main() {
    let (mut log_file, dose) = match parse_args() {
        Ok(args) => args,
        Err(error) => {
            println!("{}", USAGE);
            eprintln!("error: {}", error);
            std::process::exit(1);
        },
    };
    use chrono::SubsecRound;
    let record = format!("{} | {}", chrono::Local::now().round_subsecs(0).to_rfc3339(), dose);
    print!("appending `{}` to file...", record);
    use std::io::Write;
    if let Err(error) = writeln!(log_file, "{}", record) {
        println!("!\n");
        eprintln!("error: append failed ({})!", error);
        std::process::exit(1);
    }
    println!(" done.");
}

