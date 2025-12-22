use std::fmt;

use std::fs;
use std::fs::OpenOptions;

use std::io::{self, Write};
use std::path::PathBuf;

use rand::Rng;

const SMALL_QUESTION_CHANCE: f64 = 0.5;
const BIG_QUESTION_CHANCE: f64 = 0.5;
// const SMALL_QUESTION_CHANCE: f64 = 0.2;
// const BIG_QUESTION_CHANCE: f64 = 0.1;

const QUESTION_TYPE_TRAIL: &str = ": ";

#[derive(PartialEq)]
enum QuestionType {
    Short,
    Long,
    Answer,
    Empty
}

#[derive(Default)]
struct QuestionsCount{
    short: i64,
    long: i64,
    answer: i64,
}

#[derive(PartialEq, Default)]
struct Question{
    question: String,
}

impl From<String> for Question{
    fn from(s: String) -> Self{
        Question {
            question: s
        }
    }
}

impl fmt::Display for Question{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.question)
    }
}

trait Informative{
    fn get_type(&self) -> QuestionType;
    fn get_text(&self) -> String;
}

impl Informative for Question{
    fn get_type(&self) -> QuestionType {
        let question_type = self.question.get(..1).unwrap_or("");
        let follow_up_chars = self.question.get(1..3).unwrap_or("");

        if follow_up_chars != QUESTION_TYPE_TRAIL{
            return QuestionType::Empty;
        }

        match question_type {
            "l" => QuestionType::Long,
            "s" => QuestionType::Short,
            "a" => QuestionType::Answer,
            _ => QuestionType::Empty
        }
    }

    fn get_text(&self) -> String {
        if self.get_type() == QuestionType::Empty {
            return "".to_string();
        }

        let text = self.question.get(3..).unwrap_or("");
        text.to_string()
    }

}

fn exists_today_file(jl_files_path: PathBuf, today_file: String) -> io::Result<bool> {
    let today_file_path = jl_files_path.join(&today_file);

    if jl_files_path.is_dir() {
        for entry in fs::read_dir(jl_files_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|ext| ext.to_str()) == Some("txt") && path == today_file_path{
                return Ok(true);
            }
        }
    }
    Ok(false)
}
 
// refactor to use an iterator that only reads until endl
fn get_question_from_type(questions_file_path: &PathBuf, type_to_get: QuestionType, question_idx: i64) -> io::Result<Question> {
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

fn get_questions_counts(questions_file_path: &PathBuf) -> io::Result<QuestionsCount> {
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

fn get_question(questions_file_path: PathBuf) -> io::Result<Question> {
    // let small_questions: Vec<String> = get_question_type(questions_file_path.clone(), QuestionType::Short)?;
    // let long_questions: Vec<String>= get_question_type(questions_file_path, QuestionType::Long)?;
    let q_counts = get_questions_counts(& questions_file_path)?;

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

fn main() -> io::Result<()> {
    let mut jl_files_path = PathBuf::new();
    jl_files_path.push("./.jl");

    let today_file= chrono::offset::Local::now().format("%Y-%m-%d.txt").to_string();
    let today_file_path = jl_files_path.join(&today_file);
    let questions_file_path = jl_files_path.join("questions.txt");

    if !exists_today_file(jl_files_path, today_file)? {
        fs::write(&today_file_path, "")?;
    }

    let question_to_ask = get_question(questions_file_path)?;

    if question_to_ask == Question::default(){
        return Ok(());
    }
    println!("{}", question_to_ask.get_text());

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read line");

    let mut file = OpenOptions::new()
        .write(true)    
        .append(true)   
        .open(today_file_path)?;

    file.write_all(question_to_ask.question.as_bytes())?;
    file.write_all(answer.as_bytes())?;

    Ok(())
}
