// features to add:
// add flags
// use flags to input things: 
//      jl -d to add a description to your day
//      jl -s to show today notes 
//      jl -w to show week notes
//      jl -m to show week notes

mod question_structs;
use question_structs::{Question, Informative, QuestionType};

mod file_parsing;

use std::fs;
use std::fs::OpenOptions;

use std::io::{self, Write};
use std::path::Path;

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result, Config, EditMode};

use clap::Parser;

use home;

const JL_DIR_NAME: &str = ".jl";
const QUESTION_FILE_NAME: &str = "questions.txt";

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    description: Option<String>,

    #[arg(short, long)]
    rating: Option<f64>,
}

fn exists_today_file(jl_dir_path: &Path, today_file: &String) -> io::Result<bool> {
    let today_file_path = jl_dir_path.join(&today_file);

    if !jl_dir_path.is_dir() {
        match fs::create_dir(jl_dir_path){
            Ok(()) => (),
            Err(e) => println!("error: {}", e),
        }
    }

    if jl_dir_path.is_dir() {
        for entry in fs::read_dir(jl_dir_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|ext| ext.to_str()) == Some("txt") && path == today_file_path{
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn main() -> Result<()> {
    let args = Cli::parse();

    match args.description{
        Some(a) => {
            println!("description: {}", a);
            return Ok(());
        }
        None => (),
    }
    match args.rating{
        Some(a) => {
            println!("rating: {}", a);
            return Ok(());
        }
        None => (),
    }

    let mut jl_dir_path = home::home_dir().expect("Could not find home directory");
    jl_dir_path.push(JL_DIR_NAME);

    let today_file= chrono::offset::Local::now().format("%Y-%m-%d.txt").to_string();
    let today_file_path = jl_dir_path.join(&today_file);
    let questions_file_path = jl_dir_path.join(QUESTION_FILE_NAME);
    if !questions_file_path.exists() {
        fs::write(&questions_file_path, "l: Long question\ns: Short question\n")
            .expect("Failed to create question file\n"); 
    }

    if !exists_today_file(&jl_dir_path, &today_file)? {
        fs::write(&today_file_path, "")?;
    }

    let question_to_ask: Question = file_parsing::get_question(&questions_file_path, &today_file_path)?;

    if question_to_ask == Question::default(){
        return Ok(());
    }
    // refactor, put into function
    println!("{}", question_to_ask.get_text());

    let config = Config::builder()
        .edit_mode(EditMode::Vi) 
        .build();
    let mut rl = DefaultEditor::with_config(config)?;

    let mut file = OpenOptions::new()
        .write(true)    
        .append(true)   
        .open(&today_file_path)?;

    let mut wrote_quesiton = false;

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line == "".to_string() {
                    break
                }

                if !wrote_quesiton {
                    file.write_all(question_to_ask.question.as_bytes())?;
                    file.write_all("\n".as_bytes())?;
                    wrote_quesiton = true;
                }

                if question_to_ask.get_type() == QuestionType::Short {
                    break
                }
                else {
                    file.write_all("a: ".as_bytes())?;
                    file.write_all(line.as_bytes())?;
                    file.write_all("\n".as_bytes())?;
                }
            },
            Err(ReadlineError::Interrupted) => {
                break
            },
            Err(ReadlineError::Eof) => {
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    println!("end of file");
    Ok(())
}
