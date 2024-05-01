use std::os::fd::RawFd;
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
    handle: RawFd,
    pub name: String,
    undo_tree: UndoTree,
    yanke: String,
    saved_state: String,
}

impl File {
    pub fn new(handle: RawFd, name: String) -> File {
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
    Command,
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

    pub fn open_file(&mut self, handle: RawFd, name: String) {
        for file in &self.files {
            if handle == file.handle {
                return;
            }
        }
        let file = File::new(handle, name);
        self.files.push(file);
        self.current_screen = CurrentScreenMode::File(self.files.len() - 1);
    }

    fn save_file(&mut self) {
        if let CurrentScreenMode::File(index) = &self.current_screen {
            if let Some(text) = &self.files[*index].undo_tree.show_current_node() {
                self.files[*index].saved_state = text.clone();
           }
        }
    }

    fn quit_file(&mut self) -> std::io::Result<()> {
        let (handle, data) = if let CurrentScreenMode::File(index) = &self.current_screen {
            (self.files[*index].handle, self.files[*index].saved_state.to_owned())
        } else {(-1, "nil".to_owned())};
        if handle == -1 || &data == "nil"{
            return Ok(());
        }
        let offset = unsafe {
            lseek(handle, 0, SEEK_SET)
        };
        if offset == -1 {
            return Err(std::io::Error::last_os_error());
        }

        let bytes_written = unsafe {
            write(handle, data.as_ptr() as *const _, data.len())
        };
        if bytes_written == -1 {
            return Err(std::io::Error::last_os_error());
        }
        let truncate = unsafe {
            ftruncate(handle, data.len() as off_t)
        };
        if truncate == -1 {
            return Err(std::io::Error::last_os_error());
        }

        let len = self.files.len();
        if len == 0 {
            return Ok(())
        }
        self.files.remove(len - 1);
        
        self.current_screen = if let Some(_file) = self.files.last_mut() {
            CurrentScreenMode::File(self.files.len() - 1)
        } else {
            CurrentScreenMode::Main
        };

        Ok(())
    }
}
