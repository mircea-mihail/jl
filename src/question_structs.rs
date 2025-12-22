use std::fmt;

const QUESTION_TYPE_TRAIL: &str = ": ";

#[derive(PartialEq)]
pub enum QuestionType {
    Short,
    Long,
    Answer,
    Empty
}

#[derive(Default)]
pub struct QuestionsCount{
    pub short: i64,
    pub long: i64,
    pub answer: i64,
}

#[derive(PartialEq, Default)]
pub struct Question{
    pub question: String,
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

pub trait Informative{
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
