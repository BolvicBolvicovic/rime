use std::{
    rc::Rc,
    cell::RefCell
};

#[derive(Clone)]
pub struct Cursor {
    pub line_index: usize,
    pub line_size: usize,
    pub line: usize,
    pub max_line: usize,
}

impl Cursor {
    pub fn new(max_line: usize, line_size: usize) -> Cursor {
        Cursor {
            line_index: 0,
            line_size,
            line: 0,
            max_line
        }
    }

    pub fn move_left(&mut self) {
        if self.line_index > 0 {
            self.line_index -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.line_index < self.line_size {
            self.line_index += 1;
        }
    }

    pub fn move_up(&mut self, line_size: usize) {
        if self.line > 0 {
            self.line -= 1;
            self.line_size = line_size;
            if self.line_index > self.line_size {
                self.line_index = self.line_size;
            }
        }
    }

    pub fn move_down(&mut self, line_size: usize) {
        if self.line < self.max_line {
            self.line += 1;
            self.line_size = line_size;
            if self.line_index > self.line_size {
                self.line_index = self.line_size;
            }
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

    pub fn add_node(&mut self, text: String) {
        let node = Rc::new(RefCell::new(UndoNode {
            text,
            cursor: Cursor::new(0, 0), //TODO: Implement line size and max line
            parent: self.current.clone(),
            child: None,
        }));
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

    pub fn show_current_node(&self) -> Option<String> {
        if let Some(node) = &self.current {
            Some(node.borrow().text.clone())
        } else {
            None
        }
    }

    pub fn del_char(&mut self) {
        if let Some(node) = &mut self.current {
            node.borrow_mut().text.pop();
        }
    }

    pub fn add_char(&mut self, c: char) {
        if let Some(node) = &mut self.current {
            node.borrow_mut().text.push(c);
        } else {
            self.add_node(c.to_string());
        } 
    }

    pub fn add_newspace(&mut self) {
        let mut text = String::new();
        let mut update = false;
        if let Some(node) = &mut self.current {
            node.borrow_mut().text.push('\n');
            text = node.borrow_mut().text.clone();
        } else {
            self.add_node("\n".to_string());
        }
        if update {
            self.add_node(text);
        }
    }
}


mod tests {
    
    use super::*;

    #[test]
    fn add_one_node() {
        let mut undo_tree = UndoTree::new();
        let foo = "foo";
        undo_tree.add_node(foo.into());
        assert_eq!(Some("foo".to_string()), undo_tree.show_current_node());
    }
    
    #[test]
    fn add_two_nodes() {
        let mut undo_tree = UndoTree::new();
        let foo = "foo";
        let bar = "bar";
        undo_tree.add_node(foo.into());
        undo_tree.add_node(bar.into());
        assert_eq!(Some("bar".to_string()), undo_tree.show_current_node());
    }
    
    #[test]
    fn test_undo_no_change() {
        let mut undo_tree = UndoTree::new();
        let foo = "foo";
        let bar = "bar";
        undo_tree.add_node(foo.into());
        undo_tree.add_node(bar.into());
        undo_tree.undo();
        assert_eq!(Some("foo".to_string()), undo_tree.show_current_node());
    }
    
    #[test]
    fn test_undo_with_change() {
        let mut undo_tree = UndoTree::new();
        let foo = "foo";
        let bar = "bar";
        let foobar = "foobar";
        undo_tree.add_node(foo.into());
        undo_tree.add_node(bar.into());
        undo_tree.undo();
        undo_tree.add_node(foobar.into());
        assert_eq!(Some("foobar".to_string()), undo_tree.show_current_node());
    }
    
    #[test]
    fn test_redo_no_change() {
        let mut undo_tree = UndoTree::new();
        let foo = "foo";
        let bar = "bar";
        undo_tree.add_node(foo.into());
        undo_tree.add_node(bar.into());
        undo_tree.undo();
        undo_tree.redo();
        assert_eq!(Some("bar".to_string()), undo_tree.show_current_node());
    }
    
    #[test]
    fn test_redo_with_undo_change() {
        let mut undo_tree = UndoTree::new();
        let foo = "foo";
        let bar = "bar";
        let foobar = "foobar";
        undo_tree.add_node(foo.into());
        undo_tree.add_node(bar.into());
        undo_tree.undo();
        undo_tree.add_node(foobar.into());
        undo_tree.redo();
        assert_eq!(Some("foobar".to_string()), undo_tree.show_current_node());
        undo_tree.undo();
        undo_tree.redo();
        assert_eq!(Some("foobar".to_string()), undo_tree.show_current_node());
    }

    #[test]
    fn test_redo_undo_hard() {
        let mut undo_tree = UndoTree::new();
        let foo = "foo";
        let bar = "bar";
        let foobar = "foobar";
        undo_tree.undo();
        undo_tree.add_node(foo.into());
        undo_tree.add_node(bar.into());
        undo_tree.undo();
        undo_tree.add_node(foobar.into());
        undo_tree.redo();
        undo_tree.redo();
        assert_eq!(Some("foobar"), undo_tree.show_current_node().as_deref());
        undo_tree.undo();
        undo_tree.redo();
        assert_eq!(Some("foobar"), undo_tree.show_current_node().as_deref());
        undo_tree.add_node(foo.into());
        undo_tree.add_node(bar.into());
        undo_tree.undo();
        undo_tree.undo();
        undo_tree.redo();
        undo_tree.redo();
        assert_eq!(Some("bar"), undo_tree.show_current_node().as_deref());
        undo_tree.undo();
        undo_tree.undo();
        undo_tree.undo();
        undo_tree.undo();
        undo_tree.undo();
        undo_tree.undo();
        assert_eq!(None, undo_tree.show_current_node());
        undo_tree.redo();
        assert_eq!(Some("foo"), undo_tree.show_current_node().as_deref());
    }
}
