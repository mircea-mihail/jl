use std::fmt;

const QUESTION_TYPE_TRAIL: &str = ": ";

#[derive(PartialEq)]
pub enum QuestionType {
    Short,
    Long,
    Prompt,
    Answer,
    Info,
    Empty,
}

#[derive(PartialEq, Default, Clone, Debug)]
pub struct Question {
    pub question: String,
}

pub struct QuestionChunk {
    pub question_chunk: String,
}

pub trait ChunkParser {
    fn get_informative(&self) -> std::io::Result<Question>;
    fn get_question(&self) -> std::io::Result<Question>;
    fn get_answer(&self) -> std::io::Result<Question>;
}

impl From<String> for Question {
    fn from(s: String) -> Self {
        Question { question: s }
    }
}

impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.question)
    }
}

pub trait Informative {
    fn is_question(&self) -> bool;
    fn get_type(&self) -> QuestionType;
    fn get_type_as_str(&self) -> &str;
    fn get_text(&self) -> String;
}

impl Informative for Question {
    fn is_question(&self) -> bool {
        let question_type = self.get_type();
        if question_type != QuestionType::Empty {
            return true;
        }

        false
    }

    fn get_type(&self) -> QuestionType {
        let question_type = self.question.get(..1).unwrap_or("");
        let follow_up_chars = self.question.get(1..3).unwrap_or("");

        if follow_up_chars != QUESTION_TYPE_TRAIL {
            return QuestionType::Empty;
        }

        if question_type == "l" && self.question.get(self.question.len()-1..).unwrap_or("") == ":" {
            return QuestionType::Prompt;
        }

        match question_type {
            "l" => QuestionType::Long,
            "s" => QuestionType::Short,
            "a" => QuestionType::Answer,
            "i" => QuestionType::Info,
            _ => QuestionType::Empty,
        }
    }

    fn get_type_as_str(&self) -> &str {
        let question_type = self.question.get(..1).unwrap_or("");
        let follow_up_chars = self.question.get(1..3).unwrap_or("");

        if follow_up_chars != QUESTION_TYPE_TRAIL {
            return "";
        }

        question_type
    }

    fn get_text(&self) -> String {
        if self.get_type() == QuestionType::Empty {
            return "".to_string();
        }

        let text = self.question.get(3..).unwrap_or("");
        text.to_string()
    }
}

impl ChunkParser for QuestionChunk {
    fn get_informative(&self) -> std::io::Result<Question> {
        let question_lines: Vec<&str> = self.question_chunk.lines().collect();
        if question_lines.len() == 3 {
            return Ok(question_lines[0].to_string().into());
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "question chunk does not fit the format",
        )) 
    }

    fn get_question(&self) -> std::io::Result<Question> {
        let question_lines: Vec<&str> = self.question_chunk.lines().collect();
        if question_lines.len() == 3 {
            return Ok(question_lines[0].to_string().into());
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "question chunk does not fit the format",
        )) 
    }

    fn get_answer(&self) -> std::io::Result<Question> {
        let question_lines: Vec<&str> = self.question_chunk.lines().collect();
        if question_lines.len() == 3 {
            return Ok(question_lines[0].to_string().into());
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "question chunk does not fit the format",
        )) 
    }
}
