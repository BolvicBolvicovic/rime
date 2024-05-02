use std::{
    os::fd::{RawFd, FromRawFd},
    io::{Read, Write},
};
use libc::{ftruncate, lseek, off_t, write, SEEK_SET};

use crate::app::undotree::UndoTree;

pub enum CurrentScreenMode {
    Main,
    File(usize),
    Config,
}

impl CurrentScreenMode {
    fn unwrap(&self) -> usize {
        match self {
            CurrentScreenMode::File(index) => *index,
            _ => panic!("Try to unwrap an unwrapable mode"),
        }
    }
}

pub struct File {
    handle: std::fs::File,
    pub name: String,
    pub undo_tree: UndoTree,
    yanke: String,
    saved_state: String,
}

impl File {
    pub fn new(handle: std::fs::File, name: String) -> File {
        File {
            handle,
            name,
            undo_tree: UndoTree::new(),
            yanke: String::new(),
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
}

impl App {
    pub fn new() -> App {
        App {
            files: vec![],
            current_screen: CurrentScreenMode::Main,
            current_editing: CurrentEditing::Selecting,
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
        self.files[index].undo_tree.add_node(text.clone());
    }

    pub fn save_file(&mut self) {
        if let CurrentScreenMode::File(index) = &self.current_screen {
            if let Some(text) = &self.files[*index].undo_tree.show_current_node() {
                self.files[*index].saved_state = text.clone();
           }
        }
    }

    pub fn quit_file(&mut self) -> std::io::Result<()> {
        let mut i: i32 = if let CurrentScreenMode::File(index) = &self.current_screen {
            *index as i32
        } else {-1};
        if i == -1 {
            return Ok(());
        }
        let mut handle = self.files[TryInto::<usize>::try_into(i).unwrap()].handle.try_clone().unwrap();
        let data = self.files[TryInto::<usize>::try_into(i).unwrap()].saved_state.to_owned();

        handle.write_all(data.as_bytes())?;
        handle.set_len(data.len() as u64)?;

        self.files.remove(i.try_into().unwrap());
        self.current_screen = if let Some(_file) = self.files.last_mut() {
            CurrentScreenMode::File(self.files.len() - 1)
        } else {
            CurrentScreenMode::Main
        };

        Ok(())
    }
}
