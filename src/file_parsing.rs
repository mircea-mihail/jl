use rand::Rng;
use std::path::Path;

use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Write};

use crate::utility;

use crate::question_structs::{Informative, Question, QuestionChances, QuestionType};

use rand::seq::SliceRandom; 

fn get_question_vector(
    questions_path: &Path,
    get_type: &QuestionType,
) -> io::Result<Vec<Question>> {
    let all_questions = fs::read_to_string(questions_path)?;
    let all_questions_it = all_questions.split("\n");

    let mut q_vec: Vec<Question> = Vec::new();

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
fn get_unasked_question_vector(
    questions_path: &Path,
    get_type: &QuestionType,
    asked_questions: Vec<Question>,
) -> io::Result<Vec<Question>> {
    let all_questions = fs::read_to_string(questions_path)?;
    let all_questions_it = all_questions.split("\n");

    let mut q_vec: Vec<Question> = Vec::new();

    for question_str in all_questions_it {
        let question: Question = question_str.to_string().into();
        let question_type = question.get_type();

        if get_type == &question_type && !asked_questions.contains(&question) {
            q_vec.push(question);
        }
    }

    Ok(q_vec)
}

fn generate_jumbled_questions_file(questions_path: &Path, jumbled_questions_path: &Path) -> io::Result<()>{
    let questions_text = fs::read_to_string(questions_path)?;
    let mut questions: Vec<Question> = Vec::new();
    let mut rng = rand::rng();

    for question in questions_text.split("\n").map(|a| <Question as From<_>>::from(a.to_string())) {
        if question.is_question() {
            questions.push(question);
        }
    }
    questions.shuffle(&mut rng);

    let mut file: fs::File = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&jumbled_questions_path)?;

    for quesiton in questions {
        file.write(quesiton.to_string().as_bytes())?;
        file.write("\n".as_bytes())?;
    }
    return Ok(());
}

pub fn get_question(
    questions_path: &Path,
    today_file_path: &Path,
    question_chances: QuestionChances,
) -> io::Result<Question> {
    let jumbled_questions_path = std::path::PathBuf::from("/tmp")
        .join(utility::get_day_file_name(0));

    if !jumbled_questions_path.exists() {
        fs::write(&jumbled_questions_path, "")?;
    }
    if jumbled_questions_path.metadata().map(|m| m.len()).unwrap_or(0) == 0 {
        generate_jumbled_questions_file(&questions_path, &jumbled_questions_path)?;
    }

    let mut rnd = rand::rng();

    let question_length_sample: f32 = rnd.random();

    let question_type: QuestionType = match question_length_sample {
        c if c < question_chances.short => QuestionType::Short,
        c if c < (question_chances.short + question_chances.long) => QuestionType::Long,
        _ => return Ok(Question::default()),
    };

    let asked_questions = get_question_vector(&today_file_path, &question_type)?;
    let unasked_questions =
        get_unasked_question_vector(questions_path, &question_type, asked_questions)?;

    if unasked_questions.is_empty() {
        return Ok(Question::default());
    }

    let small_question_sample: usize = (rnd.random_range(..unasked_questions.len())) as usize;
    return Ok(unasked_questions[small_question_sample].clone());
}

pub fn exists_today_file(jl_dir_path: &Path, today_file: &String) -> io::Result<bool> {
    let today_file_path = jl_dir_path.join(&today_file);

    if !jl_dir_path.is_dir() {
        match fs::create_dir(jl_dir_path) {
            Ok(()) => (),
            Err(e) => println!("error: {}", e),
        }
    }

    if jl_dir_path.is_dir() {
        for entry in fs::read_dir(jl_dir_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|ext| ext.to_str()) == Some("txt")
                && path == today_file_path
            {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

pub fn write_question(mut file: &fs::File, question: &Question) -> io::Result<()> {
    // write info point about the question
    file.write_all(
        chrono::offset::Local::now()
            .format("i: %H:%M\n")
            .to_string()
            .as_bytes(),
    )?;

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
