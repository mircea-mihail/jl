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
mod cli;
use question_structs::{Question};
mod file_parsing;
mod utility;
mod loops;
mod pager;

use home;
use rand::Rng;
use std::fs;
use std::fs::OpenOptions;

use crate::cli::parse_days_before;

const JL_DIR_NAME: &str = ".jl";
const QUESTION_FILE_NAME: &str = "questions.txt";

fn main() -> rustyline::Result<()> {
    let mut jl_dir_path = home::home_dir().expect("Could not find home directory");
    jl_dir_path.push(JL_DIR_NAME);

    if cli::show_entries() {
        loops::view_files(&jl_dir_path)?;
        return Ok(());
    }
    let days_before_today: i64 = parse_days_before();

    let today_file = utility::get_day_file_name(days_before_today);
    let today_file_path = jl_dir_path.join(&today_file);
    let questions_file_path = jl_dir_path.join(QUESTION_FILE_NAME);

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

    if cli::parse_args(&mut question_to_ask, &mut file, &mut question_chance)? {
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
