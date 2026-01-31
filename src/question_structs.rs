// implement informative for question chunk
// do pager

use std::fmt;
use crate::cli;

const QUESTION_TYPE_TRAIL: &str = ": ";
const QUESTION_TYPE_ERROR: &str = "question chunk does not fit the format";

#[derive(PartialEq)]
pub enum QuestionType {
    Short,
    Long,
    Answer,
    Info,
    Empty,
}

pub enum LongQuesitonType {
    Regular,
    Description,
    Note,
    Rating,
}

#[derive(PartialEq, Default, Clone, Debug)]
pub struct Question {
    pub question: String,
}

pub struct QuestionChunk {
    pub question_chunk: String,
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

impl From<String> for QuestionChunk {
    fn from(s: String) -> Self {
        QuestionChunk { question_chunk: s }
    }
}

pub trait Informative {
    fn is_question(&self) -> std::io::Result<bool>;
    fn get_type(&self) -> std::io::Result<QuestionType>;
    fn get_type_as_str(&self) -> std::io::Result<&str>;
    fn get_long_type(&self) -> std::io::Result<LongQuesitonType>;
    fn get_text(&self) -> std::io::Result<String>;

}

pub trait ChunkParser {
    fn get_informative(&self) -> std::io::Result<Question>;
    fn get_question(&self) -> std::io::Result<Question>;
    fn get_answer(&self) -> std::io::Result<Question>;
}

// impl Informative for QuestionChunk {
//     fn is_question(&self) -> std::io::Result<bool> {
//         let question_type = self.get_type();

//         false
//     }

//     fn get_type(&self) -> std::io::Result<QuestionType> {
//         self.get_question()?.get_type()
//     }
// }

impl Informative for Question {
    fn is_question(&self) -> std::io::Result<bool> {
        let question_type = self.get_type()?;
        if question_type != QuestionType::Empty {
            return Ok(true);
        }

        Ok(false)
    }

    fn get_type(&self) -> std::io::Result<QuestionType> {
        let question_type = self.question.get(..1).unwrap_or("");
        let follow_up_chars = self.question.get(1..3).unwrap_or("");

        if follow_up_chars != QUESTION_TYPE_TRAIL {
            return Ok(QuestionType::Empty);
        }

        match question_type {
            "l" => Ok(QuestionType::Long),
            "s" => Ok(QuestionType::Short),
            "a" => Ok(QuestionType::Answer),
            "i" => Ok(QuestionType::Info),
            _ => Ok(QuestionType::Empty),
        }
    }

    fn get_type_as_str(&self) -> std::io::Result<&str> {
        let question_type = self.question.get(..1).unwrap_or("");
        let follow_up_chars = self.question.get(1..3).unwrap_or("");

        if follow_up_chars != QUESTION_TYPE_TRAIL {
            return Ok("");
        }

        Ok(question_type)
    }

    fn get_long_type(&self) -> std::io::Result<LongQuesitonType> {
        if self.get_type()? != QuestionType::Long {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "cannot get long type on questions that are not of type long",
                )) 
        }

        let question_string = self.get_text()?;

        if question_string == cli::DESCRIPTION_QUESTION_STR {
            return Ok(LongQuesitonType::Description);
        }
        if question_string == cli::NOTE_QUESTION_STR {
            return Ok(LongQuesitonType::Note);
        }
        if question_string == cli::RATING_QUESTION_STR {
            return Ok(LongQuesitonType::Rating);
        }

        return Ok(LongQuesitonType::Regular);
    }

    fn get_text(&self) -> std::io::Result<String> {
        if self.get_type()? == QuestionType::Empty {
            return Ok("".to_string());
        }

        let text = self.question.get(3..).unwrap_or("");
        Ok(text.to_string())
    }

}

impl ChunkParser for QuestionChunk {
    fn get_informative(&self) -> std::io::Result<Question> {
        let question_lines: Vec<&str> = self.question_chunk.lines().collect();
        if question_lines.len() >= 3 {
            return Ok(question_lines[0].to_string().into());
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            QUESTION_TYPE_ERROR,
        )) 
    }

    fn get_question(&self) -> std::io::Result<Question> {
        let question_lines: Vec<&str> = self.question_chunk.lines().collect();
        if question_lines.len() >= 3 {
            return Ok(question_lines[1].to_string().into());
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            QUESTION_TYPE_ERROR,
        )) 
    }

    fn get_answer(&self) -> std::io::Result<Question> {
        let question_lines: Vec<&str> = self.question_chunk.lines().collect();
        let mut answer = "".to_string();
        if question_lines.len() >= 3 {
            for line in &question_lines {
                answer += line;
            }

            return Ok(answer.into());
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            QUESTION_TYPE_ERROR,
        )) 
    }
}
