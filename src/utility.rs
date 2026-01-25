use std::fs;
use std::io::Write;

use crossterm::{
    cursor,
    execute, queue,
    style::{self, Stylize},
    terminal,
};

pub fn get_day_file_name(days_before: i64) -> String {
    (chrono::offset::Local::now() - chrono::Duration::days(days_before))
        .format("%Y-%m-%d.txt")
        .to_string()
}

pub fn write_content(path: &std::path::PathBuf, mut stdout: &std::io::Stdout) -> std::io::Result<()> {
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
    line_y += 2;

    let (term_width, _) = terminal::size()?;

    for line in content.lines() {
        line_x = 0;

        for mut word in line.split(" ") {
            let word_length = word.len();

            if word_length > term_width as usize {
                word = &word[..(term_width) as usize];
            }

            if word_length + line_x > term_width as usize {
                line_y += 1;
                line_x = 0;
            }

            queue!(
                stdout,
                cursor::MoveTo(line_x as u16, line_y),
                style::PrintStyledContent(word.white())
            )?;
            line_x += word_length + 1;
        }
        line_y += 1;
    }

    std::io::stdout().flush()?;

    Ok(())
}