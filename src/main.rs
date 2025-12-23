mod question_structs;
use question_structs::{Question, Informative};

mod file_parsing;

use std::fs;
use std::fs::OpenOptions;

use std::io::{self, Write};
use std::path::Path;

fn exists_today_file(jl_files_path: &Path, today_file: &String) -> io::Result<bool> {
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
 
fn main() -> io::Result<()> {
    let jl_files_path = Path::new("./.jl");

    let today_file= chrono::offset::Local::now().format("%Y-%m-%d.txt").to_string();
    let today_file_path = jl_files_path.join(&today_file);
    let questions_file_path = jl_files_path.join("questions.txt");

    if !exists_today_file(jl_files_path, &today_file)? {
        fs::write(&today_file_path, "")?;
    }
    let question_to_ask: Question = file_parsing::get_question(&questions_file_path, &today_file_path)?;

    if question_to_ask == Question::default(){
        return Ok(());
    }
    println!("{}", question_to_ask.get_text());

    let mut answer = String::new();
    io::stdin().read_line(&mut answer).expect("Failed to read line");
    answer = format!("a: {}", answer);

    let mut file = OpenOptions::new()
        .write(true)    
        .append(true)   
        .open(today_file_path)?;

    file.write_all(question_to_ask.question.as_bytes())?;
    file.write_all("\n".as_bytes())?;
    file.write_all(answer.as_bytes())?;

    Ok(())
}
