use ratatui::{style::{Color, Style, Stylize}, text::{Line, Span, Text}};

pub struct Word <'a>{
    spans: Vec<Span<'a>>,
}

impl <'a> Word<'a> {
    pub fn new<'b>(word: &'b str, _line_num: usize, cursor_index: Option<usize>) -> Word {
        let mut spans = vec![];
        if let Some(c) = cursor_index {
            match word {
                "match"  | "let"   | "pub"    | "fn"     | "enum"     | "struct" | "const" |
                "mut"    | "ref"   | "return" | "break"  | "static"   | "Self"   | "self"  |
                "super"  | "trait" | "type"   | "unsafe" | "use"      | "async"  | "where" |
                "dyn"    | "while" | "as"     | "await"  | "continue" | "crate"  | "else"  |
                "extern" | "if"    | "in"     | "loop"   | "impl"     | "for"    | "mod"   |
                "test"   | ".."    => {
                    let iterator = word.to_string();
                    for (i, ch) in iterator.chars().enumerate() {
                        if i == c {
                            spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::Rgb(183, 65, 14)).bg(Color::LightYellow).bold()));
                        } else {
                            spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::LightYellow).bold()));
                        }
                    }
                },
                "false" | "true" | "Some" | "None" | "Ok" | "Err" => { 
                    let iterator = word.to_string();
                    for (i, ch) in iterator.chars().enumerate() {
                        if i == c {
                            spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::Rgb(183, 65, 14)).bg(Color::LightMagenta)));
                        } else {
                            spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::LightMagenta)));
                        }
                    }
                },
                pattern if pattern.starts_with("&'") || pattern.starts_with('\'') => {
                    let iterator = word.to_string();
                    for (i, ch) in iterator.chars().enumerate() {
                        if i == c {
                            spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::Rgb(183, 65, 14)).bg(Color::Yellow)));
                        } else {
                            spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::Yellow)));
                        }
                    }
                },
                pattern if pattern.ends_with('!') => {
                    let iterator = word.to_string();
                    for (i, ch) in iterator.chars().enumerate() {
                        if i == c {
                            spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::Rgb(183, 65, 14)).bg(Color::Blue).bold()));
                        } else {
                            spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::Blue).bold()));
                        }
                    }
                },
                _ => {
                    let iterator = word.to_string();
                    for (i, ch) in iterator.chars().enumerate() {
                        match ch {
                        '{' | '}' | '(' | ')' | '[' | ']' | '.' | ';' | ':' | ',' =>  if i == c {
                                spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::Rgb(183, 65, 14)).bg(Color::White)));
                            } else {
                                spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::White)));
                            },
                        '+' | '=' | '*' | '-' | '/' | '&' | '<' | '>' | '#' | '?' | '|' => if i == c {
                                spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::Rgb(183, 65, 14)).bg(Color::LightYellow)));
                            } else {
                                spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::LightYellow)));
                            },
                        _ => if i == c {
                                spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::Rgb(183, 65, 14)).bg(Color::LightCyan)));
                            } else {
                                spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::LightCyan)));
                            },
                        }
                    }
                },
            }
        } else {
            match word {
                "match"  | "let"   | "pub"    | "fn"     | "enum"     | "struct" | "const" |
                "mut"    | "ref"   | "return" | "break"  | "static"   | "Self"   | "self"  |
                "super"  | "trait" | "type"   | "unsafe" | "use"      | "async"  | "where" |
                "dyn"    | "while" | "as"     | "await"  | "continue" | "crate"  | "else"  |
                "extern" | "if"    | "in"     | "loop"   | "impl"     | "for"    | "mod"   |
                "test"   | ".."    => {
                    spans.push(Span::styled(word.to_string(), Style::default().fg(Color::LightYellow).bold()));
                },
                "false" | "true" | "Some" | "None" | "Ok" | "Err" => { 
                    spans.push(Span::styled(word.to_string(), Style::default().fg(Color::LightMagenta)));
                },
                pattern if pattern.starts_with("&'") || pattern.starts_with('\'') => {
                    spans.push(Span::styled(word.to_string(), Style::default().fg(Color::Yellow)));
                },
                pattern if pattern.ends_with('!') => {
                    spans.push(Span::styled(word.to_string(), Style::default().fg(Color::Blue).bold()));
                },
                _ => {
                    let iterator = word.to_string();
                    for ch in iterator.chars() {
                        match ch {
                        '{' | '}' | '(' | ')' | '[' | ']' | '.' | ';' | ':' | ',' => spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::White))),
                        '+' | '=' | '*' | '-' | '/' | '&' | '<' | '>' | '#' | '?' | '|' => spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::LightYellow))),
                        _ => spans.push(Span::styled(ch.to_string(), Style::default().fg(Color::LightCyan))),
                        }
                    }
                },
            }
        }
        Word {
            spans,
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
                line_num.push(Line::from(Span::styled((num + 1).to_string(), Style::default().fg(Color::LightCyan).bold())));
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
        let mut previous_word_index = 0;
        let mut found = false;
        while let Some(word) = split_line.next() {
            let word_index = line[previous_word_index..].find(word).unwrap();
            let word_len = word.len();
            for i in 0..word_index {
                if let Some(c) = index && previous_word_index + i == c {
                    into_spans.push(Span::styled(" ", Style::default().bg(Color::Rgb(183, 65, 14))));
                }
                else {
                    into_spans.push(Span::styled(" ", Style::default()));
                }
            }
            let c_index = if let Some(cli) = index && !found {
                let cwi = cli as i32 - previous_word_index as i32;
                if cwi >= word_index as i32 && cwi < (word_index + word_len) as i32 {
                    found = true;
                    Some((cwi - word_index as i32).try_into().unwrap())
                } else { None }
            } else { None };
            previous_word_index += word_len + word_index;
            let word = Word::new(word, line_num, c_index);
            for span in word.spans {
                into_spans.push(span);
            }
        }
        if let Some(cli) = index {
            if cli == line.len() {
                into_spans.push(Span::styled(" ", Style::default().bg(Color::Rgb(183, 65, 14))));
            }
        }
        Line::from(into_spans)
    }
}
