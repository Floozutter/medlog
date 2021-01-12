const USAGE: &str = "\
usage: medlog <log_file> <dose_mg>
example: medlog estradiol.log 2
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
    let dose_mg = match args.next() {
        Some(dose_mg) => dose_mg,
        None => return Err("missing argument <dose_mg>!".into()),
    };
    let dose_mg = match dose_mg.parse::<u32>() {
        Ok(dose_mg) => dose_mg,
        Err(error) => return Err(format!(
            "can't parse argument <dose_mg>: `{}` ({})!",
            dose_mg, error
        ).into()),
    };
    Ok((log_file, dose_mg))
}

fn main() {
    let (mut log_file, dose_mg) = match parse_args() {
        Ok(args) => args,
        Err(error) => {
            eprintln!("{}\nerror: {}", USAGE, error);
            std::process::exit(1);
        },
    };
    use chrono::SubsecRound;
    let record = format!("{} | {} mg", chrono::Local::now().round_subsecs(0), dose_mg);
    println!("appending `{}` to file...", record);
    use std::io::Write;
    if let Err(error) = writeln!(log_file, "{}", record) {
        eprintln!("error: append failed ({})!", error);
        std::process::exit(1);
    }
}

