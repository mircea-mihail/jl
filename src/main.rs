// features to add:
// add flags
// use flags to input things: 
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
    /// Talk about how your day was
    #[arg(
        short,
        long,
        num_args = 0..=1,
        default_missing_value = "No description provided"
    )]
    description: Option<String>,

    /// Give a short update during the day
    #[arg(short, long, num_args = 0..=1)]
    update: Option<String>,

    /// Rate your day out of 10 (can be any number)
    #[arg(short, long, num_args = 0..=1)]
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

fn write_question(mut file: &fs::File, question: &Question) -> io::Result<()>{
    file.write_all(question.get_type_as_str().as_bytes())?;
    file.write_all(": ".as_bytes())?;
    file.write_all(chrono::offset::Local::now().format("%H:%M ").to_string().as_bytes())?;
    file.write_all(question.get_text().as_bytes())?;
    file.write_all("\n".as_bytes())?;

    Ok(())
}

fn write_answer(mut file: &fs::File, answer: &String) -> io::Result<()> {
    file.write_all("a: ".as_bytes())?;
    file.write_all(answer.as_bytes())?;
    file.write_all("\n".as_bytes())?;

    Ok(())
} 

fn run_input_loop(question: Question, file: &fs::File) -> Result<()> {
    println!("{}", question.get_text());

    let config = Config::builder()
        .edit_mode(EditMode::Vi) 
        .build();
    let mut rl = DefaultEditor::with_config(config)?;

    let mut wrote_quesiton = false;

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line == "".to_string() {
                    break
                }
                if !wrote_quesiton {
                    write_question(&file, &question)?;
                    wrote_quesiton = true;
                }

                write_answer(&file, &line)?;

                if question.get_type() == QuestionType::Short {
                    break
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

    Ok(())
}

fn get_answer_from_args(question: &mut Question, file: &fs::File) -> io::Result<bool> {
    let args = Cli::parse();

    match args.description{
        Some(a) => {
            *question = "l: Talk about how your day was".to_string().into();

            if a != "No description provided" {
                write_question(&file, &question)?;
                write_answer(&file, &a)?;
                return Ok(true);
            }
        }
        None => (),
    }
    match args.rating{
        Some(a) => {
            println!("rating: {}", a);
            *question = "s: Rate your day out of ten".to_string().into();
        }
        None => (),
    }

    Ok(false)
}

fn main() -> Result<()> {
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

    let file: fs::File = OpenOptions::new()
        .write(true)    
        .append(true)   
        .open(&today_file_path)?;

    let mut question_to_ask: Question = Question::default();

    if get_answer_from_args(&mut question_to_ask, &file)? {
        return Ok(());
    }

    if question_to_ask == Question::default() {
        question_to_ask = file_parsing::get_question(&questions_file_path, &today_file_path)?;
    }

    if question_to_ask == Question::default() {
        return Ok(());
    }

    run_input_loop(question_to_ask, &file)?;

    Ok(())
}
