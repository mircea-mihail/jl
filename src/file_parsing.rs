use rand::Rng;
use std::path::Path;

use std::io::{self, Write};
use std::fs;

use crate::question_structs::{Question, QuestionType, Informative};

const SHORT_QUESTION_CHANCE: f64 = 0.5;
const LONG_QUESTION_CHANCE: f64 = 0.5;
// const SHORT_QUESTION_CHANCE: f64 = 0.0;
// const LONG_QUESTION_CHANCE: f64 = 1.0;

fn get_question_vector(questions_path: &Path, get_type: &QuestionType) -> io::Result<Vec<Question>>{
    let all_questions = fs::read_to_string(questions_path)?;
    let all_questions_it = all_questions.split("\n");

    let mut q_vec:Vec<Question>= Vec::new();

    for question_str in all_questions_it {
        let question: Question = question_str.to_string().into();
        let question_type = question.get_type();

        if get_type == &question_type {
            q_vec.push(question);
        }
    }

    Ok(q_vec)
}

// returns a vector of type that only has different questions than the ones in asked_questions
fn get_unasked_question_vector(questions_path: &Path, get_type: &QuestionType, asked_questions: Vec<Question>) -> io::Result<Vec<Question>>{
    let all_questions = fs::read_to_string(questions_path)?;
    let all_questions_it = all_questions.split("\n");

    let mut q_vec:Vec<Question>= Vec::new();

    for question_str in all_questions_it {
        let question: Question = question_str.to_string().into();
        let question_type = question.get_type();

        if get_type == &question_type && !asked_questions.contains(&question) {
            q_vec.push(question);
        }
    }

    Ok(q_vec)
}

pub fn get_question(questions_path: &Path, today_file_path: &Path) -> io::Result<Question> {
    let mut rnd = rand::rng();

    let question_length_sample: f64 = rnd.random();

    let question_type: QuestionType = match question_length_sample{
        c if c < SHORT_QUESTION_CHANCE 
            => QuestionType::Short,
        c if c < (SHORT_QUESTION_CHANCE + LONG_QUESTION_CHANCE) 
            => QuestionType::Long,
        _ => return Ok(Question::default()),
    };

    let asked_questions = get_question_vector(&today_file_path, &question_type)?;
    let unasked_questions = get_unasked_question_vector(questions_path, &question_type, asked_questions)?;

    if unasked_questions.is_empty() {
        return Ok(Question::default());
    }

    let small_question_sample: usize = (rnd.random_range(..unasked_questions.len())) as usize;
    return Ok(unasked_questions[small_question_sample].clone());
}

pub fn exists_today_file(jl_dir_path: &Path, today_file: &String) -> io::Result<bool> {
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

            if path.extension().and_then(|ext| ext.to_str()) == Some("txt") && path == today_file_path {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

pub fn write_question(mut file: &fs::File, question: &Question) -> io::Result<()>{
    // write info point about the question
    file.write_all(chrono::offset::Local::now().format("i: %H:%M\n").to_string().as_bytes())?;

    // write the actual question
    file.write_all(question.get_type_as_str().as_bytes())?;
    file.write_all(": ".as_bytes())?;
    file.write_all(question.get_text().as_bytes())?;
    file.write_all("\n".as_bytes())?;

    Ok(())
}

pub fn write_answer(mut file: &fs::File, answer: &String) -> io::Result<()> {
    file.write_all("a: ".as_bytes())?;
    file.write_all(answer.as_bytes())?;
    file.write_all("\n".as_bytes())?;

    Ok(())
}
