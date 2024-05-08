use ratatui::text::Span;

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
}

pub struct Word <'a>{
    head_index: usize,
    spans: Vec<Span<'a>>,
    definition: AstDefinition,
}

impl <'a> Tree<'a> {
    //pub fn new<'b>(word: &'b str) -> Word {
    //TODO:
    //}
}

pub struct Tree<'a> {
    raw_text: String,
    lines: Vec<(Word<'a>, AstDefinition)>,
}

impl <'a> Tree<'a> {
    pub fn new(raw_text: String) -> Tree<'a> {
        let lines = vec![];
        Tree {
            raw_text,
            lines,
        }
    }

    pub fn parse(&'a mut self) {
        let lines = self.raw_text.lines();
        for line in lines {
            let split_line = line.split_whitespace();
            for word in split_line {
    //            self.lines.push(Word::new(word));
            }
        }
    }
}
