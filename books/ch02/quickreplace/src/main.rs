use regex::Regex;
use std::env;
use std::fs;
use text_colorizer::Colorize;

fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(data) => data,
        Err(e) => {
            eprintln!(
                "{} failed to read from file '{}' : {:?}",
                "Error:".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        }
    };

    let replaced = match replace(&args.target, &args.replacement, &data) {
        Ok(replaced) => replaced,
        Err(e) => {
            eprintln!(
                "{} failed to replace '{}' with '{}' : {:?}",
                "Error:".red().bold(),
                args.target,
                args.replacement,
                e
            );
            std::process::exit(1);
        }
    };

    match fs::write(&args.output, replaced) {
        Ok(_) => (),
        Err(e) => {
            eprintln!(
                "{} failed to write to file '{}' : {:?}",
                "Error:".red().bold(),
                args.output,
                e
            );
            std::process::exit(1);
        }
    }
}

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn print_usage() {
    eprintln!(
        "{} - change occurrences of one string into another",
        "quickreplace".green()
    );
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected 4, got {}",
            "Error:".red(),
            args.len()
        );
        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let re = Regex::new(target)?;
    Ok(re.replace_all(text, replacement).to_string())
}
