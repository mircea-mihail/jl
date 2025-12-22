use rand::Rng;
use std::path::Path;

use std::fs;
use std::io;

use crate::question_structs::{Question, QuestionsCount, QuestionType, Informative};

const SMALL_QUESTION_CHANCE: f64 = 0.5;
const BIG_QUESTION_CHANCE: f64 = 0.5;
// const SMALL_QUESTION_CHANCE: f64 = 0.2;
// const BIG_QUESTION_CHANCE: f64 = 0.1;

// refactor to use an iterator that only reads until endl
fn get_question_from_type(questions_file_path: &Path, type_to_get: QuestionType, question_idx: i64) -> io::Result<Question> {
    let all_questions = fs::read_to_string(questions_file_path)?;
    let all_questions_it = all_questions.split("\n");

    let mut q_idx = 0;
    for question_str in all_questions_it {
        let question: Question = question_str.to_string().into();
        let question_type = question.get_type();

        if type_to_get == question_type {
            if q_idx == question_idx{
                return Ok(question);
            }
            q_idx += 1;
        }
    }

    Ok(Question::default())
}

fn get_questions_counts(questions_file_path: &Path) -> io::Result<QuestionsCount> {
    let mut q_count: QuestionsCount = QuestionsCount{short:0, long: 0, answer: 0};

    let all_questions = fs::read_to_string(questions_file_path)?;
    let all_questions_it = all_questions.split("\n");

    for question_str in all_questions_it {
        let question: Question = question_str.to_string().into();
        let question_type = question.get_type();

        match question_type{
            QuestionType::Long => q_count.long += 1,
            QuestionType::Short => q_count.short += 1,
            QuestionType::Answer => q_count.answer += 1,
            _ => continue,
        }
    }

    Ok(q_count)
}

pub fn get_question(questions_file_path: &Path) -> io::Result<Question> {
    // let small_questions: Vec<String> = get_question_type(questions_file_path.clone(), QuestionType::Short)?;
    // let long_questions: Vec<String>= get_question_type(questions_file_path, QuestionType::Long)?;
    let q_counts = get_questions_counts(& questions_file_path.to_path_buf())?;

    let mut rnd = rand::rng();

    let question_length_sample: f64 = rnd.random();
    let small_question_sample: i64 = (rnd.random::<f64>() * (q_counts.short as f64)) as i64;
    let long_question_sample: i64 = (rnd.random::<f64>() * (q_counts.long as f64)) as i64;

    match question_length_sample{
        c if c < SMALL_QUESTION_CHANCE 
            => return get_question_from_type(&questions_file_path, QuestionType::Short, small_question_sample),
        c if c < (SMALL_QUESTION_CHANCE + BIG_QUESTION_CHANCE) 
            => return get_question_from_type(&questions_file_path, QuestionType::Long, long_question_sample),
        _ => (),
    }

    Ok(Question::default())
}

