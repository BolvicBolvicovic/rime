use std::{
    rc::Rc,
    cell::RefCell
};

#[derive(Clone)]
pub struct Cursor {
    index: usize,
    max: usize,
}

impl Cursor {
    pub fn new(index: usize, max: usize) -> Cursor {
        Cursor {
            index,
            max,
        }
    }

    pub fn move_left(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.index < self.max {
            self.index += 1;
        }
    }

    pub fn move_up(&mut self, text: String) {
        let mut previous_nl = 0;
        let text = text.as_bytes();
        if self.index == self.max {
            self.index -= 1;
        } else if self.index + 1 < self.max {
            self.index += 1;
        }
        while self.index - previous_nl > 0 && text[self.index - previous_nl] != b'\n' {
            previous_nl += 1;
        }
        while self.index > 0 && text[self.index] != b'\n' {
            self.index -= 1;
        }
        if self.index == 0 {
            return;
        }
        self.index -= 1;
        let mut line_len = 0; 
        while self.index - line_len > 0 && text[self.index - line_len] != b'\n' {
            line_len += 1;
        }
        while self.index > 0 && text[self.index] != b'\n' && (line_len as i32 - previous_nl as i32) >= 0 {
            self.index -= 1;
            line_len -= 1;
        }
    }

    pub fn move_down(&mut self, text: String) {
        let mut previous_nl = 0;
        let text = text.as_bytes();
        if self.index > 0 {
            self.index -= 1;
        }
        while self.index - previous_nl > 0 && text[self.index - previous_nl] != b'\n' {
            previous_nl += 1;
        }
        if self.index as i32 - previous_nl as i32 == 0 {
            previous_nl += 1;
        }
        if text[self.index] == b'\n' {
            self.index += 1;
        }
        while self.max - 1 > self.index && text[self.index] != b'\n' {
            self.index += 1;
        }
        if self.index == self.max - 1 {
            return;
        }
        self.index += 1;
        let mut line_len = 0; 
        while self.index < self.max - 1 && text[self.index] != b'\n' && (previous_nl as i32 - line_len as i32) > 0 {
            self.index += 1;
            line_len += 1;
        }
    }
}

#[derive(Clone)]
pub struct UndoNode {
    pub text: String,
    pub cursor: Cursor,
    pub parent: Option<Rc<RefCell<UndoNode>>>,
    pub child: Option<Rc<RefCell<UndoNode>>>
}

impl UndoNode {
    pub fn new(text: String, parent: Option<Rc<RefCell<UndoNode>>>, cursor: Cursor) -> UndoNode {
        UndoNode {
            text,
            cursor,
            parent,
            child: None,
        }
    }
}

pub struct UndoTree {
    pub current: Option<Rc<RefCell<UndoNode>>>,
    current_child: Option<Rc<RefCell<UndoNode>>>
}

impl UndoTree {
    pub fn new() -> UndoTree {
        UndoTree {
            current: None,
            current_child: None,
        }
    }

    pub fn add_node(&mut self, text: String, cursor: Cursor) {
        let node = Rc::new(RefCell::new(UndoNode::new(text, self.current.clone(), cursor)));
        if let Some(current) = &mut self.current {
            current.borrow_mut().child = Some(node.clone());
            self.current = Some(node);
            self.current_child = None;
        } else {
            self.current = Some(node);
        }
    }

    pub fn undo(&mut self) {
        if self.current.is_some() {
            self.current_child = self.current.clone();
            let parent = self.current.as_ref().unwrap().borrow().parent.clone();
            self.current = parent;
        }
    }

    pub fn redo(&mut self) {
        if let Some(node) = &self.current_child {
            self.current = Some(node.clone());
        }
        if let Some(current) = &self.current {
            self.current_child = current.borrow().child.clone();
        }
    }

    pub fn show_current_node(&self) -> Option<(String, usize)> {
        if let Some(node) = &self.current {
            Some((node.borrow().text.clone(), node.borrow().cursor.index))
        } else {
            None
        }
    }

    pub fn del_char(&mut self) {
        if let Some(node) = &mut self.current {
            let index = node.borrow().cursor.index;
            if index as i32 - 1 >= 0 {
                let _ = node.borrow_mut().text.remove(index - 1);
                node.borrow_mut().cursor.max -= 1;
                self.move_cursor_left();
            }
        }
    }

    pub fn add_char(&mut self, c: char) {
        if let Some(node) = &mut self.current {
            let cursor_index = node.borrow().cursor.index;
            node.borrow_mut().text.insert(cursor_index, c);
            node.borrow_mut().cursor.max += 1;
            self.move_cursor_right();
        } else {
            self.add_node(c.to_string(), Cursor::new(0, 1));
        } 
    }

    pub fn add_newspace(&mut self) {
        self.add_char('\n');
        let mut text = String::new();
        let mut cursor = Cursor::new(0, 1);
        if let Some(node) = &self.current {
            text = node.borrow().text.clone();
            cursor = node.borrow().cursor.clone();
        }
        self.add_node(text, cursor);
    }

    pub fn move_cursor_up(&mut self) {
        if let Some(node) = &mut self.current {
            let text = node.borrow().text.clone();
            node.borrow_mut().cursor.move_up(text);
        }
    }
    
    pub fn move_cursor_down(&mut self) {
        if let Some(node) = &mut self.current {
            let text = node.borrow().text.clone();
            node.borrow_mut().cursor.move_down(text);
        }
    }
    
    pub fn move_cursor_left(&mut self) {
        if let Some(node) = &mut self.current {
            node.borrow_mut().cursor.move_left();
        }
    }
    
    pub fn move_cursor_right(&mut self) {
        if let Some(node) = &mut self.current {
            node.borrow_mut().cursor.move_right();
        }
    }
}
