use ratatui::{style::{Color, Style, Stylize}, text::{Line, Span, Text}};

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
        if let Some(c) = cursor_index {
            match word {
                "match" | "let" | "pub" | "fn" | "enum" | "struct" | "const" | "+" | "=" | "*" | "-" | "/" => {
                    let iterator = word.to_string();
                    for (i, ch) in iterator.chars().enumerate() {
                        if i == c {
                            spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::Rgb(183, 65, 14)).bg(Color::LightYellow).bold()));
                        } else {
                            spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::LightYellow).bold()));
                        }
                    }
                },
                _ => {
                    let iterator = word.to_string();
                    for (i, ch) in iterator.chars().enumerate() {
                        if i == c {
                            spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::Rgb(183, 65, 14)).bg(Color::LightCyan)));
                        } else {
                            spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::LightCyan)));
                        }
                    }
                },
            }
        } else {
            match word {
                "match" | "let" | "pub" | "fn" | "enum" | "struct" | "const" | "+" | "=" | "*" | "-" | "/" => {
                    spans.push(Span::styled(word, Style::default().fg(Color::LightYellow).bold()));
                },
                _ => spans.push(Span::styled(word, Style::default().fg(Color::LightCyan))),
            }
        }
        Word {
            line_num,
            spans,
            definition,
        }
    }
}

pub struct Tree<'a> {
    pub lines: Vec<Line<'a>>,
    pub line_num: Vec<Line<'a>>,
}

impl <'a> Tree<'a> {
    pub fn new(raw_text: &'a str, cursor_index: usize) -> Tree<'a> {
        let mut lines = vec![];
        let mut line_num = vec![];
        let raw_lines = raw_text.lines();
        let mut cursor_line_index = 0;
        let mut found = false;
        let cursor_index = cursor_index;
        for (num, line) in raw_lines.enumerate() {
            let line_len = line.len() + 1;
            cursor_line_index += line_len;
            if cursor_line_index > cursor_index && !found {
                let index = cursor_index + line_len - cursor_line_index;
                line_num.push(Line::from(Span::styled((num + 1).to_string(), Style::default().fg(Color::LightCyan))));
                lines.push(Tree::<'a>::build_line(line, num, Some(index)));
                found = true;
            } else {
                lines.push(Tree::<'a>::build_line(line, num, None));
                line_num.push(Line::from(Span::styled((num + 1).to_string(), Style::default().fg(Color::Rgb(183, 65, 14)))));
            }
        }
        Tree {
            lines,
            line_num,
        }
    }

    pub fn into_linetext(&'a self) -> Text {
        Text::from(self.lines.clone())
    }

    pub fn into_numtext(&'a self) -> Text {
        Text::from(self.line_num.clone())
    }

    fn build_line(line: &'a str, line_num: usize, index: Option<usize>) -> Line {
        let mut split_line = line.split_whitespace();
        let mut into_spans = vec![];
        let line_len = line.len();
        let mut previous_word_index = 1;
        while let Some(word) = split_line.next() {
            let word_index = line_len - split_line.remainder().unwrap_or("").len() - word.len();
            let word_len = word.len();
            for i in 0..(word_index - previous_word_index) {
                if let Some(c) = index && previous_word_index + i - 1 == c {
                    into_spans.push(Span::styled(" ", Style::default().bg(Color::Rgb(183, 65, 14))));
                }
                else {
                    into_spans.push(Span::styled(" ", Style::default()));
                }
            }
            let c_index = if let Some(i) = index && word_index as i32 + word_len as i32 - i as i32 + 1 >= 0 {Some(word_index + i - 1)} else {None};
            previous_word_index = word_len + word_index;
            let word = Word::new(word, line_num, c_index);
            for span in word.spans {
                into_spans.push(span);
            }
        }
        Line::from(into_spans)
    }
}
