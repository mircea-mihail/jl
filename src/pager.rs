use std::{io::Write};

use crossterm::{
    cursor, execute, queue,
    style::{self, Stylize},
    terminal,
};

use crate::question_structs::{
    ChunkParser, Informative, PromptQuestionType, QuestionChunk, QuestionType
};

pub fn format_content(content: &String) -> std::io::Result<String> {
    let mut notes: Vec<String> = Vec::new();
    let mut descriptions: Vec<String> = Vec::new();
    let mut ratings: Vec<String> = Vec::new();
    let mut long_questions: Vec<String> = Vec::new();
    let mut short_questions: Vec<String> = Vec::new();
    let mut this_chunk_str: String = "".to_string();

    let mut lines  = content.lines().peekable();

    while let Some(line) = lines.next() {
        if !line.is_empty() {
            this_chunk_str += line;
            this_chunk_str += "\n";
    
        }
        if line.is_empty() || lines.peek().is_none(){
            let this_chunk = QuestionChunk::from(this_chunk_str.clone());
            let mut chunk_iter = this_chunk.get_answer()?.into_iter();

            if this_chunk.get_prompt_type()? == PromptQuestionType::Rating {
                while let Some(question) = chunk_iter.next()  {
                    ratings.push(question.get_text()?);
                    eprintln!("got rating {}", question.get_text()?);
                }
            } 
            else if this_chunk.get_prompt_type()? == PromptQuestionType::Description {
                while let Some(question) = chunk_iter.next()  {
                    descriptions.push(format!("{}", question.get_text()?));
                }
            }
            else if this_chunk.get_prompt_type()? == PromptQuestionType::Note {
                while let Some(question) = chunk_iter.next()  {
                    notes.push(format!("{}", question.get_text()?));
                }
            }
            else if this_chunk.get_type()? == QuestionType::Long{
                while let Some(question) = chunk_iter.next()  {
                    long_questions.push(format!("    {}", this_chunk.get_question()?.get_text()?));
                    long_questions.push(format!("    {}\n", question.get_text()?));
                }
            }

            this_chunk_str = "".to_string();
        }
   }
    let mut return_content = "".to_string();

    if ! ratings.is_empty() {
        return_content += "rating: ";
        return_content += ratings.join("->").as_str();
    }

    if !descriptions.is_empty() {
        return_content += "\n\ndescription: \n    ";
        return_content += descriptions.join("\n").as_str();
    }

    if !notes.is_empty() {
        return_content += "\n\nnotes: \n    ";
        return_content += notes.join("\n").as_str();
    }

    if !long_questions.is_empty() || !short_questions.is_empty() {
        return_content += "\n\nother questions: \n";
        return_content += long_questions.join("\n").as_str();
    }

    Ok(return_content)
}

pub fn parse_display_text(content: &String) -> std::io::Result<Vec<String>> {
    let (term_width, _) = terminal::size()?;

    let mut terminal_lines: Vec<String> = Vec::new();
    let mut terminal_line: String = "".to_string();
    let mut line_x;

    for line in content.lines() {
        line_x = 0;

        for mut word in line.split(" ") {
            let mut word_length = word.len();

            if word_length > term_width as usize {
                word = &word[..(term_width) as usize];
                word_length = word.len();
            }

            if word_length + line_x + 1 > term_width as usize {
                terminal_lines.push(terminal_line.clone());
                terminal_line = "".to_string();

                line_x = 0;
            }

            terminal_line += word;
            terminal_line += " ";

            line_x += word_length + 1;
        }
        terminal_lines.push(terminal_line.clone());
        terminal_line = "".to_string();
    }

    terminal_lines.push(terminal_line.clone());
    Ok(terminal_lines)
}

pub fn write_display_content(
    path: &std::path::PathBuf,
    height_index: usize,
    terminal_lines: &Vec<String>,
    mut stdout: &std::io::Stdout,
) -> std::io::Result<()> {
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    let (_, term_height) = terminal::size()?;
    let mut line_y = 0;
    let mut path_str = "";

    if let Some(stem_os) = path.file_stem() {
        if let Some(stem_str) = stem_os.to_str() {
            path_str = stem_str;
        }
    }
    queue!(
        stdout,
        cursor::MoveTo(0 as u16, line_y),
        style::PrintStyledContent(path_str.white())
    )?;

    let init_line_y = 2;
    line_y = init_line_y;

    let mut line_idx = 0;
    for line in terminal_lines {
        if line_idx >= height_index
            && line_idx < height_index + term_height as usize - init_line_y as usize 
        {
            queue!(
                stdout,
                cursor::MoveTo(0 as u16, line_y),
                style::PrintStyledContent(line.clone().white())
            )?;
            line_y += 1;
        }
        line_idx += 1;
    }

    std::io::stdout().flush()?;

    Ok(())
}
