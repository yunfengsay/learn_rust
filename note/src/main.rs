extern crate chrono;
use chrono::prelude::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::fs::OpenOptions;

struct InputContent {
    command: String,
    content: String,
}

static FILE_PATH:&str = ".content.note";

fn main() -> io::Result<()> {
    if !is_file_exist(FILE_PATH) {
        let mut output: File = File::create(FILE_PATH)?;
        // write!(output, "Rust\n:)");

    }
    let command_len = 2;
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush();
        io::stdin().read_line(&mut input).unwrap();
        let input_str = input.trim();
        let mut splitter = input_str.splitn(2, " ");
     
        if splitter.clone().count() < command_len {
            println!("command type: {{ Command }} {{ Content }} "); // rust这么跳过大括号真是奇葩的恶心
            continue;
        };

        let command = splitter.next().unwrap();
        let content = splitter.next().unwrap();
        match command.to_lowercase().as_ref() {
            "push" => insert_str(content),
            "pop" => pop_str(content),
            // "search" => search_str(content), 
            _ => println!("no such command {}", command),
        }

        // let input: File = File::open(path)?;
        // let buffered: BufReader<File> = BufReader::new(input);

        // for line in buffered.lines().map(|x| x.unwrap()) {
        //     // line: String     x:Result<String, Error>
        //     println!("{}", line);
        // }
    }
}

fn is_file_exist(src:&str) -> bool {
    Path::new(src).exists()
}

fn insert_str(content: &str) {
    let local: DateTime<Local> = Local::now();
    let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(FILE_PATH)
            .unwrap();
    let content = format!("{} | {}", local, content);
    if let Err(e) = writeln!(file, "{}", content) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn pop_str(content: &str) {
    match search_str(content) {
        None => println!("{}", "no such content"),
        Some(val) => {
            println!("{:?}", val.0);
            let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(FILE_PATH)
            .unwrap();
            
        },
    }
}
fn search_str(content: &str) -> Option<(usize, String)> {
    let input: File = File::open(FILE_PATH).unwrap();
    let buffered: BufReader<File> = BufReader::new(input);
    for (i, line) in buffered.lines().map(|x| x.unwrap()).enumerate()  {
        // line: String     x:Result<String, Error>
        if line.contains(content){
            println!("find {}", line);
            return Some((i, line))
        }
    }
    None
}