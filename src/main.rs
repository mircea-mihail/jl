// todo and features to add:
//
// make the vim extension go to normal mode when pressing kj
// make cursor different in insert and normal mode (| for insert, box for insert)
//
// when displaying summaries, only show the final rating given (and maybe use the other ones to show how the day evolved)
//      if someone has a rating of 5 in the morning but a 8 in the afternoon maybe show how the day improved and print the updates (if available)
// use flags to input things:
//      jl -w to show week notes
//      jl -m to show months notes highlights
//      show a table/graph for short questions for the week
//      and maybe something interactive with browsing longer notes/ a compilation of notes?

mod question_structs;
use crossterm::terminal::DisableLineWrap;
use question_structs::{Informative, Question, QuestionType};
mod file_parsing;
mod utility;

use home;
use rand::Rng;
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Write};

use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::{Config, DefaultEditor, EditMode};

const JL_DIR_NAME: &str = ".jl";
const QUESTION_FILE_NAME: &str = "questions.txt";

const DEFAULT_DESCRIPTION: &str = "No description provided";
const DEFAULT_NOTE: &str = "No note provided";
const DEFAULT_RATING: &str = "1212.1212";
const DEFAULT_SOMETIMES: &str = "true";

const QUESTION_CHANCE: f64 = 0.5;

use minus::{dynamic_paging, MinusError, Pager};
use std::{
    thread::{spawn, sleep},
    time::Duration
};

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

    /// Add a short note during the day
    #[arg(
        short,
        long,
        num_args = 0..=1,
        default_missing_value = DEFAULT_NOTE
    )]
    note: Option<String>,

    /// Rate your day out of 10 (can be any number)
    #[arg(
        short,
        long,
        num_args = 0..=1,
        default_missing_value = DEFAULT_RATING
    )]
    rating: Option<f64>,

    /// Lower chances of a question being asked
    #[arg (
        short,
        long,
        num_args = 0..=1,
        default_missing_value = DEFAULT_SOMETIMES
    )]
    sometimes: Option<bool>,

    /// Update journal from x days ago
    #[arg(short, long, num_args = 1)]
    update: Option<i64>,
}

fn parse_args(
    args: Cli,
    question: &mut Question,
    file: &mut fs::File,
    question_chance: &mut f64,
) -> io::Result<bool> {
    match args.description {
        Some(a) => {
            *question = "l: Add a description about the day:".to_string().into();

            if a != DEFAULT_DESCRIPTION {
                file.write_all("\n".as_bytes())?;
                file_parsing::write_question(&file, &question)?;
                file_parsing::write_answer(&file, &a)?;
                return Ok(true);
            }
        }
        None => (),
    }
    match args.note {
        Some(a) => {
            *question = "l: Add a note during the day:".to_string().into();

            if a != DEFAULT_NOTE {
                file.write_all("\n".as_bytes())?;
                file_parsing::write_question(&file, &question)?;
                file_parsing::write_answer(&file, &a)?;
                return Ok(true);
            }
        }
        None => (),
    }
    match args.rating {
        Some(a) => {
            *question = "s: Add a rating for your day out of ten:".to_string().into();

            if a != DEFAULT_RATING.parse::<f64>().unwrap() {
                file.write_all("\n".as_bytes())?;
                file_parsing::write_question(&file, &question)?;
                file_parsing::write_answer(&file, &a.to_string())?;
                return Ok(true);
            }
        }
        None => (),
    }
    args.sometimes.map(|s: bool| {
        if s == DEFAULT_SOMETIMES.parse::<bool>().unwrap() {
            *question_chance = QUESTION_CHANCE;
        }
    });

    Ok(false)
}

fn run_input_loop(question: Question, file: &mut fs::File, write_question_gap: bool) -> rustyline::Result<()> {
    println!("{}", question.get_text());

    let config = Config::builder().edit_mode(EditMode::Vi).build();
    let mut rl = DefaultEditor::with_config(config)?;

    let mut wrote_quesiton = false;

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line == "".to_string() {
                    break;
                }
                if !wrote_quesiton {
                    if write_question_gap {
                        file.write_all("\n".as_bytes())?;
                    }

                    file_parsing::write_question(&file, &question)?;
                    wrote_quesiton = true;
                }

                file_parsing::write_answer(&file, &line)?;

                if question.get_type() == QuestionType::Short {
                    break;
                }
            }
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), MinusError> {
    let args = Cli::parse();

    let mut days_before_today: i64 = 0;
    match args.update {
        Some(s) => {
            days_before_today = s;
            // println!("Update entry for {}", utility::get_day_file_name(days_before_today).split(".").next().unwrap_or("unknown date."));
            println!("Update entry for {}:", (chrono::offset::Local::now() - chrono::Duration::days(days_before_today)).format("%d %b %Y") .to_string());
        }
        None => (),
    }

    let mut jl_dir_path = home::home_dir().expect("Could not find home directory");
    jl_dir_path.push(JL_DIR_NAME);

    let today_file = utility::get_day_file_name(days_before_today);
    let today_file_path = jl_dir_path.join(&today_file);
    let questions_file_path = jl_dir_path.join(QUESTION_FILE_NAME);

    ////////////////////////////////////////////////////////////////////////////////////
    let dir_files = fs::read_dir(jl_dir_path)?;
    let mut journal_paths: Vec<std::path::PathBuf> = Vec::new();
    for file in dir_files {
        let path = file?.path();

        if let Some(stem) = path.file_stem() {
            let string_split_stem = stem.to_string_lossy();
            let stem_vec: Vec<&str> = string_split_stem.split("-").collect();
            if stem_vec.len() == 3 && stem_vec[0].len() == 4 && stem_vec[1].len() == 2 && stem_vec[2].len() == 2 {
                journal_paths.push(path);
            }
        }
    }
    journal_paths.sort();
    let current_idx = journal_paths.len() - 1;
    println!("jl paths:{:?}", journal_paths);
    
    let day_file_content= fs::read_to_string(&journal_paths[current_idx])?;

    // Initialize the pager
    let pager = Pager::new();
    // Run the pager in a separate thread
    let pager2 = pager.clone();
    let pager_thread = spawn(move || dynamic_paging(pager2));

    for line in day_file_content.lines() {
        pager.push_str(line.to_string() + "\n")?;
    }
    pager_thread.join().unwrap()?;
    return Ok(());
    ////////////////////////////////////////////////////////////////////////////////////

    let mut write_question_gap = true;

    if !questions_file_path.exists() {
        fs::write(
            &questions_file_path,
            "l: Long question\ns: Short question\n",
        )
        .expect("Failed to create question file\n");
    }
    if !file_parsing::exists_today_file(&jl_dir_path, &today_file)? {
        fs::write(&today_file_path, "")?;
        write_question_gap = false;
    }
    if fs::metadata(&today_file_path)?.len() == 0 {
        write_question_gap = false;
    }

    let mut file: fs::File = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&today_file_path)?;

    let mut question_to_ask: Question = Question::default();
    let mut question_chance: f64 = 1.0;

    if parse_args(args, &mut question_to_ask, &mut file, &mut question_chance)? {
        return Ok(());
    }

    let mut rng = rand::rng();

    if question_to_ask == Question::default() && rng.random::<f64>() < question_chance{
        question_to_ask = file_parsing::get_question(&questions_file_path)?;
    }

    if question_to_ask == Question::default() {
        return Ok(());
    }

    run_input_loop(question_to_ask, &mut file, write_question_gap);

    Ok(())
}
