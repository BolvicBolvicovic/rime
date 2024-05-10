use ratatui::{style::{Color, Style}, text::{Line, Span}};

pub enum AstDefinition {
    Keyword,
    Variable,
    Function,
    Macro,
    Type,
    StringStyle,
    Trait,
    Constant,
    Lifetime,
    SelfKeyWord,
    Unknown,
}

pub struct Word <'a>{
    line_num: usize,
    spans: Vec<Span<'a>>,
    definition: AstDefinition,
}

impl <'a> Word<'a> {
    pub fn new<'b>(word: &'b str, line_num: usize, cursor_index: Option<usize>) -> Word {
        let mut spans = vec![];
        let definition = AstDefinition::Unknown;
        //TODO: Build each word
        Word {
            line_num,
            spans,
            definition,
        }
    }
}

pub struct Tree<'a> {
    raw_text: String,
    cursor_index: usize,
    lines: Vec<Line<'a>>,
    line_num: Vec<Line<'a>>,
}

impl <'a> Tree<'a> {
    pub fn new(raw_text: String, cursor_index: usize) -> Tree<'a> {
        let lines = vec![];
        let line_num = vec![];
        Tree {
            raw_text,
            cursor_index,
            lines,
            line_num,
        }
    }

    pub fn parse(&'a mut self) {
        let lines = self.raw_text.lines();
        let mut cursor_line_index = 0;
        let mut found = false;
        let cursor_index = self.cursor_index;
        for (num, line) in lines.enumerate() {
            let line_len = line.len() + 1;
            cursor_line_index += line_len;
            if cursor_line_index > cursor_index && !found {
                let index = cursor_index + line_len - cursor_line_index;
                self.line_num.push(Line::from(Span::styled((num + 1).to_string(), Style::default().fg(Color::LightCyan))));
                self.lines.push(Tree::<'a>::build_line(line, num, Some(index)));
                found = true;
            } else {
                self.lines.push(Tree::<'a>::build_line(line, num, None));
                self.line_num.push(Line::from(Span::styled((num + 1).to_string(), Style::default().fg(Color::Rgb(183, 65, 14)))));
            }
        }
    }

    fn build_line(line: &'a str, line_num: usize, index: Option<usize>) -> Line {
        let mut split_line = line.split_whitespace();
        let mut into_spans = vec![];
        let line_len = line.len();
        let mut previous_word_index = if let Some(w) = split_line.nth(0) {w.len()} else {0};
        while let Some(word) = split_line.next() {
            let word_index = line_len - split_line.remainder().unwrap().len();
            let word_len = word.len();
            for i in 0..(word_index - previous_word_index) {
                if let Some(c) = index && previous_word_index + i == c {
                    into_spans.push(Span::styled(" ", Style::default().bg(Color::Rgb(183, 65, 14))));
                }
                else {
                    into_spans.push(Span::styled(" ", Style::default()));
                }
            }
            let c_index = if let Some(i) = index && word_index as i32 + word_len as i32 - i as i32 >= 0 {Some(i - word_index)} else {None};
            previous_word_index = word_len + word_index;
            let word = Word::new(word, line_num, c_index);
            for span in word.spans {
                into_spans.push(span);
            }
        }
        Line::from(into_spans)
    }
}
