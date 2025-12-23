use rand::Rng;
use std::path::Path;

use std::fs;
use std::io;

use crate::question_structs::{Question, QuestionType, Informative};

const SHORT_QUESTION_CHANCE: f64 = 0.5;
const LONG_QUESTION_CHANCE: f64 = 0.5;
// const SMALL_QUESTION_CHANCE: f64 = 0.2;
// const BIG_QUESTION_CHANCE: f64 = 0.1;

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

// fn get_question_from_vector(get_type: QuestionType, question_idx: i64) -> Question {
// }

pub fn get_question(questions_path: &Path, today_file_path: &Path) -> io::Result<Question> {
    // let small_questions: Vec<String> = get_question_type(questions_path.clone(), QuestionType::Short)?;
    // let long_questions: Vec<String>= get_question_type(questions_path, QuestionType::Long)?;

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

