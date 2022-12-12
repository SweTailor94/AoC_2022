use std::collections::HashMap;
use itertools::Itertools;
use crate::input::InputParser;

pub struct ElfFile {
    pub name: String,
    pub size: u32,
}

pub struct ElfFolder {
    pub name: String,
    // Absolute path name
    pub folders: Vec<String>,
    pub files: Vec<ElfFile>,
}

pub struct FileSystem2 {
    pub folders: HashMap<String, ElfFolder>,
    pub current_dir: String,
}

impl FileSystem2 {
    pub fn sum_folders_smaller_than(&self, size:u32) -> u32 {
        let sizes = self.get_folder_sizes("/");
        sizes.iter().filter(|s|s <= &&size).sum()
    }

    pub fn get_folder_sizes(&self, path : &str) -> Vec<u32> {
        if let Some(my_folder) = self.folders.get(path) {
            let mut my_foldersize = 0;
            for f in &my_folder.files {
                my_foldersize += f.size;
            }
            let mut sizes: Vec<u32> = Vec::new();
            for p in &my_folder.folders
            {
                let p_sizes = self.get_folder_sizes(&p);
                my_foldersize += p_sizes[0];
                sizes.extend(p_sizes);
            }
            let mut result = Vec::new();
            result.push(my_foldersize);
            result.extend(sizes);

            result
        }else{
            panic!("Can't find folder {}",path);
        }
    }
}

impl InputParser for FileSystem2 {
    fn parse_line(&mut self, line: &String) -> anyhow::Result<()> {
        let line_parts = line.split(" ").collect_vec();
        match line_parts[0] {
            "$" => {
                match line_parts[1] {
                    "cd" => {
                        match line_parts[2] {
                            ".." => { // Go up one level
                                self.current_dir = up_one_level(&self.current_dir);
                            }
                            "/" => { // Go to root
                                self.current_dir = "/".to_string();
                                if !self.folders.contains_key(&self.current_dir) {
                                    self.folders.insert(self.current_dir.clone(),
                                                        ElfFolder {
                                                            name: self.current_dir.clone(),
                                                            folders: Vec::new(),
                                                            files: Vec::new(),
                                                        });
                                }
                            }
                            dir => { // Go down to directory named dir
                                self.current_dir = down_one_level(&self.current_dir, dir);
                                if !self.folders.contains_key(&self.current_dir) {
                                    self.folders.insert(self.current_dir.clone(),
                                                        ElfFolder {
                                                            name: self.current_dir.clone(),
                                                            folders: Vec::new(),
                                                            files: Vec::new(),
                                                        });
                                }
                            }
                        }
                    }
                    "ls" => {
                        // following lines are a listing of current directory
                    }
                    _ => {} // ignore unknown commands
                }
            }
            "dir" => {
                if let Some(current) = self.folders.get_mut(&self.current_dir) {
                    current.folders.push(down_one_level(&self.current_dir, line_parts[1]));
                } else {
                    panic!("Parse error: current dir does not exist")
                }
            }
            fsize => {
                if let Some(current) = self.folders.get_mut(&self.current_dir) {
                    current.files.push(ElfFile{
                        name: line_parts[1].to_string(),
                        size: fsize.parse().unwrap(),
                    });
                } else {
                    panic!("Parse error: current dir does not exist")
                }
            }
        }

        Ok(())
    }
}

fn up_one_level(path: &String) -> String {
    let parts = path.split("/").collect_vec();
    if parts.len() == 1 {
        "/".to_string()
    } else {
        parts.iter().take(parts.len() - 1).fold("".to_string(), |path, folder| if path.ends_with("/") {
            format!("{}{}",path,folder)
        }else {
            format!("{}/{}", path, folder)
        }).to_string()
    }
}

fn down_one_level(path: &String, folder: &str) -> String {
    if path.ends_with("/") {
        format!("{}{}",path,folder)
    }else {
        format!("{}/{}", path, folder)
    }
}

#[cfg(test)]
mod test {
    use crate::day7::day7_alt_model::{down_one_level, up_one_level};

    #[test]
    pub fn test_path_stuff(){

        assert_eq!(down_one_level(&"/".to_string(),"apa"), "/apa");
        assert_eq!(down_one_level(&"/a".to_string(),"e"), "/a/e");
        assert_eq!(up_one_level(&"/a/e".to_string()), "/a");
        assert_eq!(up_one_level(&"/a".to_string()), "/");
    }
}

