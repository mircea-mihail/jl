// todo and features to add:
//
// make the vim extension go to normal mode when pressing kj
// make cursor different in insert and normal mode (| for insert, box for insert)
//
// use flags to input things:
//      jl -w to show week notes
//      jl -m to show months notes highlights
//      show a table/graph for short questions for the week
//      and maybe something interactive with browsing longer notes/ a compilation of notes?

mod cli;
mod question_structs;
use question_structs::Question;
mod file_parsing;
mod loops;
mod pager;
mod utility;

use home;
use std::path::PathBuf;
use rand::Rng;
use std::fs;
use std::fs::OpenOptions;

use crate::cli::parse_days_before;

const JL_DIR_NAME: &str = ".jl";
const QUESTION_FILE_NAME: &str = "questions.txt";

use crossterm::{execute, terminal};
use rustyline::error::ReadlineError;
use std::io;

fn main() -> rustyline::Result<()> {
    let mut jl_dir_path = home::home_dir().expect("Could not find home directory");
    jl_dir_path.push(JL_DIR_NAME);

    if cli::show_entries() {
        match loops::view_files(&jl_dir_path) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                let mut stdout = io::stdout();
                execute!(stdout, terminal::LeaveAlternateScreen)?;
                terminal::disable_raw_mode()?;

                return Err(ReadlineError::Io(io::Error::new(
                    io::ErrorKind::Other,
                    e.to_string(),
                )));
            }
        }
    }
    let days_before_today: i64 = parse_days_before();

    let today_file = utility::get_day_file_name(days_before_today);
    let today_file_path = jl_dir_path.join(&today_file);
    let questions_file_path = jl_dir_path.join(QUESTION_FILE_NAME);

    let mut write_question_gap = true;

    if !file_parsing::exists_today_file(&jl_dir_path, &today_file)? {
        fs::write(&today_file_path, "")?;
        write_question_gap = false;
    }
    if fs::metadata(&today_file_path)?.len() == 0 {
        write_question_gap = false;
    }

    // copy current dir questions file in ~/.jl folder or create an empty one otherwise if not existing there
    let mut this_dir_questions_path = PathBuf::new();
    this_dir_questions_path.push("./");
    this_dir_questions_path.push(   QUESTION_FILE_NAME);

    if this_dir_questions_path.exists(){ 
        fs::copy(&this_dir_questions_path, &questions_file_path)?;
    }
    else {
        if !questions_file_path.exists() {
            fs::write(
                &questions_file_path,
                "l: Long question\ns: Short question\n",
            )
            .expect("Failed to create question file\n");
        }
    }

    let mut file: fs::File = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&today_file_path)?;

    let mut question_to_ask: Question = Question::default();
    let mut question_chance: f64 = 1.0;

    if cli::parse_args(
        &mut question_to_ask,
        &mut file,
        &mut question_chance,
        &write_question_gap,
    )? {
        return Ok(());
    }

    let mut rng = rand::rng();

    if question_to_ask == Question::default() && rng.random::<f64>() < question_chance {
        question_to_ask = file_parsing::get_question(&questions_file_path)?;
    }

    if question_to_ask == Question::default() {
        return Ok(());
    }

    loops::get_input(question_to_ask, &mut file, write_question_gap)?;

    Ok(())
}
