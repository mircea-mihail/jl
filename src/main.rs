// todo and features to add:
// fix endline bug (jl -u or jl -d when empty prints endlines)
// make the vim extension go to normal mode when pressing kj
// make cursor different in insert and normal mode (| for insert, box for insert)
// cool infopoint data: hour: minute, country, weather, degrees C
// when displaying summaries, only show the final rating given (and maybe use the other ones to show how the day evolved)
//      if someone has a rating of 5 in the morning but a 8 in the afternoon maybe show how the day improved and print the updates (if available)
// use flags to input things: 
//      jl -w to show week notes
//      jl -m to show months notes
//      think about how to show a table for short questions for the week 
//      and maybe something interactive with browsing longer notes?

mod question_structs;
use question_structs::{Question, Informative, QuestionType};
mod file_parsing;

use std::fs;
use std::fs::OpenOptions;
use home;
use std::io::{self, Write};

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result, Config, EditMode};
use clap::Parser;

use crate::question_structs::QuestionChances;

const JL_DIR_NAME: &str = ".jl";
const QUESTION_FILE_NAME: &str = "questions.txt";

const DEFAULT_DESCRIPTION: &str = "No description provided";
const DEFAULT_UPDATE: &str = "No update provided";
const DEFAULT_RATING: &str = "1212.1212";
const DEFAULT_SOMETIMES: &str = "true";

const DEFAULT_SHORT_CHANCE: f32 = 0.4;
const DEFAULT_LONG_CHANCE: f32 = 0.6;

const RARE_SHORT_CHANCE: f32 = 0.1;
const RARE_LONG_CHANCE: f32 = 0.1;

#[derive(Parser)]
struct Cli {
    /// Talk about how your day was
    #[arg(
        short,
        long,
        num_args = 0..=1,
        default_missing_value = DEFAULT_DESCRIPTION
    )]
    description: Option<String>,

    /// Give a short update during the day
    #[arg(
        short,
        long,
        num_args = 0..=1,
        default_missing_value = DEFAULT_UPDATE
    )]
    update: Option<String>,

    /// Rate your day out of 10 (can be any number)
    #[arg(
        short,
        long,
        num_args = 0..=1,
        default_missing_value = DEFAULT_RATING
    )]
    rating: Option<f64>,

    /// Use this flag in order to lower chances of a question being asked
    #[arg (
        short,
        long,
        num_args = 0..=1,
        default_missing_value = DEFAULT_SOMETIMES
    )]
    sometimes: Option<bool>,
}

fn get_answer_from_args(question: &mut Question, file: &fs::File, question_chances: &mut QuestionChances) -> io::Result<bool> {
    let args = Cli::parse();

    match args.description{
        Some(a) => {
            *question = "l: Talk about how your day was".to_string().into();

            if a != DEFAULT_DESCRIPTION {
                file_parsing::write_question(&file, &question)?;
                file_parsing::write_answer(&file, &a)?;
                return Ok(true);
            }
        }
        None => (),
    }
    match args.update{
        Some(a) => {
            *question = "l: Give an update about your day".to_string().into();

            if a != DEFAULT_UPDATE {
                file_parsing::write_question(&file, &question)?;
                file_parsing::write_answer(&file, &a)?;
                return Ok(true);
            }
        }
        None => (),
    }
    match args.rating{
        Some(a) => {
            *question = "s: Rate your day out of ten".to_string().into();

            if a != DEFAULT_RATING.parse::<f64>().unwrap() {
                file_parsing::write_question(&file, &question)?;
                file_parsing::write_answer(&file, &a.to_string())?;
                return Ok(true);
            }
        }
        None => (),
    }
    match args.sometimes {
        Some(s) => {
            if s != DEFAULT_SOMETIMES.parse::<bool>().unwrap() {
                question_chances.short = DEFAULT_SHORT_CHANCE;
                question_chances.long = DEFAULT_LONG_CHANCE;
            }
            else {
                question_chances.short = RARE_SHORT_CHANCE;
                question_chances.long = RARE_LONG_CHANCE;
            }
       }
        None => (),
    }

    Ok(false)
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
                    file_parsing::write_question(&file, &question)?;
                    wrote_quesiton = true;
                }

                file_parsing::write_answer(&file, &line)?;

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

fn main() -> Result<()> {
    let mut jl_dir_path = home::home_dir().expect("Could not find home directory");
    jl_dir_path.push(JL_DIR_NAME);

    let today_file= chrono::offset::Local::now().format("%Y-%m-%d.txt").to_string();
    let today_file_path = jl_dir_path.join(&today_file);
    let questions_file_path = jl_dir_path.join(QUESTION_FILE_NAME);

    let mut write_question_gap = true;

    if !questions_file_path.exists() {
        fs::write(&questions_file_path, "l: Long question\ns: Short question\n")
            .expect("Failed to create question file\n"); 
    }
    if !file_parsing::exists_today_file(&jl_dir_path, &today_file)? {
        fs::write(&today_file_path, "")?;
        write_question_gap = false;
    }

    let mut file: fs::File = OpenOptions::new()
        .write(true)    
        .append(true)   
        .open(&today_file_path)?;

    if write_question_gap {
        file.write_all("\n".as_bytes())?;
    }

    let mut question_to_ask: Question = Question::default();
    let mut question_chances: QuestionChances = QuestionChances {
        short: (DEFAULT_SHORT_CHANCE), long: (DEFAULT_LONG_CHANCE) 
    };

    if get_answer_from_args(&mut question_to_ask, &file, &mut question_chances)? {
        return Ok(());
    }

    if question_to_ask == Question::default() {
        question_to_ask = file_parsing::get_question(&questions_file_path, &today_file_path, question_chances)?;
    }

    if question_to_ask == Question::default() {
        return Ok(());
    }

    run_input_loop(question_to_ask, &file)?;

    Ok(())
}
