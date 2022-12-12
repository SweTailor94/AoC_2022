// model types for Day7

use std::collections::{BTreeMap, VecDeque};
use itertools::Itertools;
use crate::input::InputParser;

pub struct ElfFile{
    pub name: String,
    pub size: usize,
}

impl ElfFile {
    pub fn new(name: String, size: usize) -> ElfFile {
        ElfFile{
            name,
            size,
        }
    }
}

pub struct ElfFolder {
    pub name: String,
    pub folders: Vec<ElfFolder>,
    pub files: Vec<ElfFile>,
}


impl ElfFolder {

    pub fn new(name: String) -> ElfFolder{
        ElfFolder{
            name,
            folders: Vec::new(),
            files: Vec::new(),
        }
    }
    pub fn get_folder(&self, name: &str ) -> Option<&ElfFolder> {
        self.folders.iter().find(|f| &f.name == name)
    }

    pub fn get_folder_size(&self) -> usize {
        self.files.iter().map(|f| f.size).sum::<usize>() + self.folders.iter().map(|f| f.get_folder_size()).sum::<usize>()
    }

    pub fn add_file(&mut self, size:usize , name: String){
        if match self.files.iter().find(|f|f.name == name) {
            None => {true}
            Some(_) => {
                false
            }
        }{
            self.files.push(ElfFile::new(name,size));
        }
    }

    pub(crate) fn add_folder(&mut self, name: String) {
        if match self.folders.iter().find(|f| f.name == name){
            None => {true}
            Some(_) => {false}
        }{
            self.folders.push(ElfFolder::new(name));
        }
    }
}


pub struct FileSystem<'a>{
    pub root: ElfFolder,
    pub cur_dir: VecDeque< &'a mut ElfFolder>,
}

impl <'a> InputParser for FileSystem<'_>{
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        let line_parts = line.split(" ").collect_vec();
        match line_parts[0] {
            "$" => {
                match line_parts[1] {
                    "cd" => {
                        match line_parts[2] {
                            ".." => { // Go up one level

                            }
                            "/" => { // Go to root
                                self.cur_dir.clear();
                                let x = &mut self.root;
                                //self.cur_dir.push_back(x)
                            }
                            dir => { // Go down to directory named dir

                            }
                        }
                    }
                    "ls" => {
                        // following lines are a listing of
                    }
                    _ => {} // ignore unknown commands
                }

            }
            "dir" => {
                let current = self.cur_dir.back_mut().unwrap();
                current.add_folder(line_parts[1].to_string())
            }
            size => {
                let current = self.cur_dir.back_mut().unwrap();
                current.add_file(size.parse().unwrap(),line_parts[1].to_string());
            }
        }

        Ok(())
    }
}
