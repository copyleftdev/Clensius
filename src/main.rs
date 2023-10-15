use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use regex::Regex;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Instant;
use rayon::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let regexes = vec![
        r"[\!(,%]",
        r".{100,}",
        r"[0-9]{4,}",
        r"[0-9]{3,}$",
        r"[a-z0-9]{32}",
        r"[0-9]+[A-Z0-9]{5,}",
        r"\/.*\/.*\/.*\/.*\/.*\/.*\/",
        r"\w{8}-\w{4}-\w{4}-\w{4}-\w{12}",
        r"[0-9]+[a-zA-Z]+[0-9]+[a-zA-Z]+[0-9]+",
        r"\.(png|jpg|jpeg|gif|svg|bmp|ttf|avif|wav|mp4|aac|ajax|css|all)$",
        r"^$",
    ];

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} -wl <wordlist_file>", args[0]);
        std::process::exit(1);
    }

    let mut wordlist_file = String::new();
    let mut arg_index = 1;

    while arg_index < args.len() {
        if args[arg_index] == "-wl" {
            if arg_index + 1 < args.len() {
                wordlist_file = args[arg_index + 1].clone();
                arg_index += 2;
            } else {
                eprintln!("Missing wordlist file path after -wl option.");
                std::process::exit(1);
            }
        } else {
            eprintln!("Invalid option: {}", args[arg_index]);
            std::process::exit(1);
        }
    }

    if wordlist_file.is_empty() {
        eprintln!("Missing wordlist file path. Use -wl <wordlist_file> option.");
        std::process::exit(1);
    }

    let file = File::open(&wordlist_file)?;
    let total_lines = io::BufReader::new(file).lines().count() as u64;

    let temp_dir = format!("{}_temp", wordlist_file);
    fs::create_dir_all(&temp_dir)?;

    let style = ProgressStyle::default_spinner()
        .template("[{elapsed}] {bar:40.cyan/blue} {pos}/{len} ({percent}%)")
        .unwrap(); 

    let pb = ProgressBar::new(total_lines);
    pb.set_style(style);

    let start_time = Instant::now();

    let filtered_lines: Vec<String> = io::BufReader::new(File::open(&wordlist_file)?)
        .lines()
        .par_bridge()
        .filter(|line_result| {
            let line = line_result.as_ref().unwrap();
            !regexes.iter().any(|regex_pattern| {
                let re = Regex::new(regex_pattern).unwrap();
                re.is_match(&line)
            })
        })
        .map(|line_result| {
            pb.inc(1);
            line_result.unwrap()
        })
        .collect();

    pb.finish_and_clear();

    let elapsed_time = start_time.elapsed();
    let mut output_file = File::create(&wordlist_file)?;

    for line in &filtered_lines {
        writeln!(output_file, "{}", line)?;
    }

    fs::remove_dir_all(&temp_dir)?;

    println!("[-] Removed {} lines", total_lines - filtered_lines.len() as u64);
    println!("[-] Wordlist is now {} lines", filtered_lines.len());
    println!("[+] Done");
    println!("Elapsed time: {:?}", elapsed_time);

    Ok(())
}
