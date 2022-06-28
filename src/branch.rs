use std::fs::{read_dir, metadata, ReadDir};
use colored::*;

pub struct Branch;

#[derive(PartialEq)]
#[warn(non_camel_case_types)]
pub enum Show_mode {
    ALL,
    FILES,
    FOLDER,
}

impl Branch {
    pub fn display(path : String , padding : u128 , deepnis : u128, mode : &Show_mode ,hidden : bool) {
        if padding == deepnis { //|| !metadata(&path).unwrap().is_dir() {
            return;
        }

        let dirs = Branch::get_folder(read_dir(path.clone()).unwrap() , hidden);
        let files =  Branch::get_files(read_dir(path).unwrap() , hidden);

        match mode {
            &Show_mode::FOLDER => Branch::show_folder(dirs, padding, deepnis, mode, hidden),
            &Show_mode::FILES => Branch::show_files(files , padding),
            _ => {
                Branch::show_folder(dirs, padding, deepnis, mode, hidden);
                Branch::show_files(files, padding);
            }
        }



    }
    
    fn show_folder(dirs : Vec<String> , padding : u128 , deepnis : u128, mode : &Show_mode, hidden : bool) {
        for dir in dirs {
            Branch::printt( Branch::carf(dir.clone()), padding, (60, 170, 165), true);
            Branch::display(dir, padding + 1, deepnis, mode, hidden);
        }
    }

    fn show_files(files : Vec<String> , padding : u128) {
        for file in files {
            Branch::printt( Branch::carf(file), padding, (160, 70, 65), false);
        }
    }

    fn printt(str : String, padding : u128, color : (u8,u8,u8), folder : bool) {
        for _ in 0..padding {
            print!("     ");
        }
        if folder {
            println!("{}", format!("{}/", str).truecolor(color.0, color.1, color.2).bold());
        } else {
            println!("{}", str.truecolor(color.0, color.1, color.2));
        }
    }

    fn carf(str : String) -> String {
        let mut name = String::new();
        for c in str.chars(){
            name.push(c);
            if c == '/' {
                name.clear();
            }
        }        
        name
    }

    fn get_folder(rd : ReadDir , hidden : bool) -> Vec<String>{
        let mut new = vec![];
        for unproc_path in rd {
            let path = format!("{}", unproc_path.unwrap().path().display());
            match metadata(&path) {
                Ok(_) => {},
                Err(_) => continue
            };
            if metadata(&path).unwrap().is_dir() && !(hidden && path.contains("/.")) 
            {
                new.push(path);
            }
        }
        new
    }

    fn get_files(rd : ReadDir , hidden : bool) -> Vec<String> {
        let mut new = vec![];
        for unproc_path in rd {
            let path = format!("{}", unproc_path.unwrap().path().display());
            match metadata(&path) {
                Ok(_) => {},
                Err(_) => continue
            };
            if metadata(&path).unwrap().is_file() && !(hidden && path.contains("/.")) 
            {
                new.push(path);
            }
        }
        new
    }
}