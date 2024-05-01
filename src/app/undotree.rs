use std::{
    rc::Rc,
    cell::RefCell
};

#[derive(Clone)]
pub struct UndoNode {
    text: String,
    pub parent: Option<Rc<RefCell<UndoNode>>>,
    pub child: Option<Rc<RefCell<UndoNode>>>
}

pub struct UndoTree {
    current: Option<Rc<RefCell<UndoNode>>>,
}

impl UndoTree {
    pub fn new() -> UndoTree {
        UndoTree {
            current: None,
        }
    }

    fn add_node(&mut self, text: String) {
        let node = Rc::new(RefCell::new(UndoNode {
            text,
            parent: self.current.clone(),
            child: None,
        }));
        if let Some(current) = &mut self.current {
            current.borrow_mut().child = Some(node.clone());
            self.current = Some(node);
        } else {
            self.current = Some(node);
        }
    }

    fn undo(&mut self) {
        let mut update = false;
        if let Some(_current) = &self.current {
            update = true;
        }
        if update {
            self.current = if let Some(current) = &self.current {
                current.borrow().parent.clone()
            } else { None };
        }
    }

    fn redo(&mut self) {
        let mut update = false;
        if let Some(node) = &self.current {
            if let Some(_child) = &node.borrow().child {
                update = true;
            }
        }
        if update {
            self.current = if let Some(current) = &self.current {
                current.borrow().child.clone()
            } else { None };
        }
    }

    pub fn show_current_node(&self) -> Option<String> {
        if let Some(node) = &self.current {
            Some(node.borrow().text.clone())
        } else {
            None
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
        assert_eq!(Some("foobar".to_string()), undo_tree.show_current_node());
        undo_tree.undo();
        undo_tree.redo();
        assert_eq!(Some("foobar".to_string()), undo_tree.show_current_node());
        undo_tree.add_node(foo.into());
        undo_tree.add_node(bar.into());
        undo_tree.undo();
        undo_tree.undo();
        undo_tree.redo();
        undo_tree.redo();
        assert_eq!(Some("bar".to_string()), undo_tree.show_current_node());
        undo_tree.undo();
        undo_tree.undo();
        undo_tree.undo();
        undo_tree.undo();
        undo_tree.undo();
        undo_tree.undo();
        assert_eq!(None, undo_tree.show_current_node());
    }
}
