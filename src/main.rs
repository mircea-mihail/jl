use std::fmt;

use std::fs;
use std::fs::OpenOptions;

use std::io::{self, Write};
use std::path::PathBuf;

use rand::Rng;

// const SMALL_QUESTION_CHANCE: f64 = 0.5;
// const BIG_QUESTION_CHANCE: f64 = 0.5;
const SMALL_QUESTION_CHANCE: f64 = 0.2;
const BIG_QUESTION_CHANCE: f64 = 0.1;

const QUESTION_TYPE_TRAIL: &str = ": ";

struct Question{
    question: String
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
    fn get_type(&self) -> String;
    fn get_text(&self) -> String;
}

impl Informative for Question{
    fn get_type(&self) -> String {
        let question_type = self.question.get(..1).unwrap_or("");
        let follow_up_chars = self.question.get(1..3).unwrap_or("");

        if follow_up_chars != QUESTION_TYPE_TRAIL{
            return "".to_string();
        }

        question_type.to_string()
    }

    fn get_text(&self) -> String {
        if self.get_type() == "".to_string() {
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

// fn get_small_questions(questions_file_path: PathBuf) -> Vec<String> {

// }

fn get_question(questions_file_path: PathBuf) -> io::Result<String> {
    let mut small_questions: Vec<String>= Vec::new();
    let mut long_questions: Vec<String>= Vec::new();

    let all_questions = fs::read_to_string(questions_file_path)?;
    let all_questions_it = all_questions.split("\n");

    for question in all_questions_it {
        // if question[:1]
        // small_questions.append();
        let question_type = question.get(..1).unwrap_or("");
        let question_text = question.get(3..).unwrap_or("").to_string();
        match question_type{
            "s" => small_questions.push(question_text),
            "l" => long_questions.push(question_text),
            _ => continue, 
        }
    }
    let mut rnd = rand::rng();

    let question_length_sample: f64 = rnd.random();
    let small_question_sample: usize = (rnd.random::<f64>() * (small_questions.len() as f64)) as usize;
    let long_question_sample: usize = (rnd.random::<f64>() * (long_questions.len() as f64)) as usize;

    match question_length_sample{
        c if c < SMALL_QUESTION_CHANCE 
            => return Ok(small_questions[small_question_sample].clone()),
        c if c < (SMALL_QUESTION_CHANCE + BIG_QUESTION_CHANCE) 
            => return Ok(long_questions[long_question_sample].clone()),
        _ => (),
    }

    Ok("".to_string())
}

fn main() -> io::Result<()> {
    let question: Question = "m: hello".to_string().into();
    let q_type = question.get_type();
    let q_text = question.get_text();
    println!("{}", q_type);
    println!("{}", q_text);
    
    return Ok(());
    let mut jl_files_path = PathBuf::new();
    jl_files_path.push("./.jl");

    let today_file= chrono::offset::Local::now().format("%Y-%m-%d.txt").to_string();
    let today_file_path = jl_files_path.join(&today_file);
    let questions_file_path = jl_files_path.join("questions.txt");

    if !exists_today_file(jl_files_path, today_file)? {
        fs::write(&today_file_path, "")?;
    }

    let question_to_ask = get_question(questions_file_path)?;

    if question_to_ask == ""{
        return Ok(());
    }
    println!("{}", question_to_ask);

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read line");

    let mut file = OpenOptions::new()
        .write(true)    
        .append(true)   
        .open(today_file_path)?;

    file.write_all(question_to_ask.as_bytes())?;
    file.write_all(answer.as_bytes())?;

    Ok(())
}
