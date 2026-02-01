use std::{io::Write};

use crossterm::{
    cursor, execute, queue,
    style::{self, Stylize},
    terminal,
};

use crate::question_structs::{
    ChunkParser, Informative, LongQuesitonType, Question, QuestionChunk, QuestionType,
};

pub fn format_content(content: &String) -> std::io::Result<String> {
    eprintln!("trying to format content..");
    // let mut chunk_vec: Vec<QuestionChunk> = Vec::new();
    // let mut notes: Vec<QuestionChunk> = Vec::new();
    let mut descriptions: Vec<String> = Vec::new();
    // let mut ratings: Vec<QuestionChunk> = Vec::new();
    let mut this_chunk_str: String = "".to_string();

    let mut lines  = content.lines().peekable();

    while let Some(line) = lines.next() {
        if !line.is_empty() {
            this_chunk_str += line;
            this_chunk_str += "\n";
    
        }
        if line.is_empty() || lines.peek().is_none(){
            let this_chunk = QuestionChunk::from(this_chunk_str.clone());

            if this_chunk.get_type()? == QuestionType::Long
                && this_chunk.get_long_type()? == LongQuesitonType::Description 
            {
                let mut chunk_iter = this_chunk.get_answer()?.into_iter();
                if let Some(question) = chunk_iter.next(){
                    descriptions.push(format!("    {}", question.get_text()?));
                }

                while let Some(question) = chunk_iter.next()  {
                    descriptions.push(question.get_text()?);
                }
            }

            this_chunk_str = "".to_string();
        }
   }

    let mut return_content = "".to_string();
    return_content += descriptions.join("\n").as_str();

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
            && line_idx < height_index + term_height as usize - init_line_y as usize + 1
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
