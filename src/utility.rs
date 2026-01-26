use std::fs;
use std::io::Write;

use crossterm::{
    cursor, execute, queue,
    style::{self, Stylize},
    terminal,
};

pub fn get_day_file_name(days_before: i64) -> String {
    (chrono::offset::Local::now() - chrono::Duration::days(days_before))
        .format("%Y-%m-%d.txt")
        .to_string()
}

pub fn write_display_content(
    path: &std::path::PathBuf,
    height_index: usize,
    mut stdout: &std::io::Stdout,
) -> std::io::Result<()> {
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    let mut line_x = 0;
    let mut line_y = 0;

    let content = fs::read_to_string(path)?;
    let mut path_str = "";

    if let Some(stem_os) = path.file_stem() {
        if let Some(stem_str) = stem_os.to_str() {
            path_str = stem_str;
        }
    }
    queue!(
        stdout,
        cursor::MoveTo(line_x as u16, line_y),
        style::PrintStyledContent(path_str.white())
    )?;

    let (term_width, term_height) = terminal::size()?;

    let mut terminal_lines: Vec<String> = Vec::new();
    let mut terminal_line: String = "".to_string();

    for line in content.lines() {
        line_x = 0;

        for mut word in line.split(" ") {
            let mut word_length = word.len();

            if word_length > term_width as usize {
                word = &word[..(term_width) as usize];
                word_length = word.len();
            }

            if word_length + line_x > term_width as usize {
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
    let init_line_y = 2;
    line_y = init_line_y;

    let mut line_idx = 0;
    for line in terminal_lines {
        if line_idx > height_index
            && line_idx < height_index + term_height as usize - init_line_y as usize + 1
        {
            queue!(
                stdout,
                cursor::MoveTo(0 as u16, line_y),
                style::PrintStyledContent(line.white())
            )?;
            line_y += 1;
        }
        line_idx += 1;
    }

    std::io::stdout().flush()?;

    Ok(())
}
