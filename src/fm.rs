use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
};

use crate::parser::Statement;

pub struct FileManager {
    file_stack: VecDeque<String>,
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            file_stack: VecDeque::new(),
        }
    }

    pub fn get_next_file_contents(&mut self) -> String {
        let file = self.file_stack.pop_front().unwrap();
        // TODO( relative paths);
        FileManager::read_file_contents(file)
    }

    pub fn is_empty(&self) -> bool {
        self.file_stack.is_empty()
    }

    pub fn extend_file_stack(&mut self, parsed: &Vec<Statement>) {
        let include_statements: Vec<Statement> = parsed
            .iter()
            .filter_map(|statement| {
                if matches!(statement, Statement::IncludeStatement(_)) {
                    Some(statement.clone())
                } else {
                    None
                }
            })
            .collect();

        for include in include_statements {
            if let Statement::IncludeStatement(file_name) = include {
                self.file_stack.insert(0, file_name);
            }
        }
    }

    pub fn read_file_contents(file_name: String) -> String {
        std::fs::read_to_string(file_name).unwrap()
    }

    pub fn resolve_path(current_file: &Path, include_path: &str) -> PathBuf {
        if Path::new(include_path).is_absolute() {
            PathBuf::from(include_path)
        } else {
            current_file.parent().unwrap().join(include_path)
        }
    }
}
