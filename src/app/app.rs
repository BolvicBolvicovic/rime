use std::io::{Read, Seek, Write};

use crate::{app::undotree::UndoTree, Cursor};

pub enum CurrentScreenMode {
    Main,
    File(usize),
    Config,
}

pub struct File {
    handle: std::fs::File,
    pub name: String,
    pub undo_tree: UndoTree,
    saved_state: String,
}

impl File {
    pub fn new(handle: std::fs::File, name: String) -> File {
        File {
            handle,
            name,
            undo_tree: UndoTree::new(),
            saved_state: String::new(),
        }
    }
}

pub enum CurrentEditing {
    Page,
    Command(char),
    Selecting,
    Listening(char),
}

pub struct App {
    pub files: Vec<File>,
    pub current_screen: CurrentScreenMode,
    pub current_editing: CurrentEditing,
    yanke: String,
}

impl App {
    pub fn new() -> App {
        App {
            files: vec![],
            current_screen: CurrentScreenMode::Main,
            current_editing: CurrentEditing::Selecting,
            yanke: String::new(),
        }
    }

    pub fn open_file(&mut self, handle: std::fs::File, name: String) {
        let file = File::new(handle, name);
        self.files.push(file);
        let index = self.files.len() - 1;
        self.current_screen = CurrentScreenMode::File(index);
        let mut text = String::new();
        let _ = self.files[index].handle.read_to_string(&mut text);
        self.files[index].saved_state = text.clone();
        self.files[index].undo_tree.add_node(text.clone(), Cursor::new(0, text.len()));
    }

    pub fn save_file(&mut self) {
        if let CurrentScreenMode::File(index) = &self.current_screen {
            if let Some((text, _)) = &self.files[*index].undo_tree.show_current_node() {
                self.files[*index].saved_state = text.clone();
           }
        }
    }

    pub fn quit_file(&mut self) -> std::io::Result<()> {
        let i: i32 = if let CurrentScreenMode::File(index) = &self.current_screen {
            *index as i32
        } else {-1};
        if i == -1 {
            return Ok(());
        }
        let i : usize = i.try_into().unwrap();

        let mut handle = self.files[i].handle.try_clone().unwrap();
        let data = self.files[i].saved_state.to_owned();

        handle.rewind()?;
        handle.write_all(data.as_bytes())?;
        handle.set_len(data.len() as u64)?;
        
        self.files.remove(i);
        self.current_screen = if self.files.len() != 0 {
            if i as i32 -1 > -1 {
                CurrentScreenMode::File(i - 1)
            } else {
                CurrentScreenMode::File(i)
            }
        } else {
            CurrentScreenMode::Main
        };

        Ok(())
    }
}
