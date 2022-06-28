
use std::{process::exit, fs::metadata};
use colored::Colorize;

pub struct CLI;
pub struct Branch_arg {
    pub path : String,
    pub padding : u128,
    pub deepnis : u128,
    pub mode : u8,
    pub hidden : bool,
}

impl CLI {
    pub fn handle_args(args : &Vec<String>) -> Branch_arg {
        let mut branch_arg = Branch_arg {
            path : ".".to_string(),
            padding : 0,
            deepnis : 3, //u128::MAX,
            mode : 0,
            hidden : true
        };
        if args[1] != "".to_string() {
            match metadata(&args[1]) {
                Ok(_) => {
                    if metadata(&args[1]).unwrap().is_dir() {
                        branch_arg.path = args[1].clone();
                    }
                },
                Err(_) => ()
            }   
        }
     

        for (i,command) in args.iter().enumerate() {
            match command.as_str() {
                "--help" => CLI::print_help(),
                "-h" => CLI::print_help(),
                "-d" => { if CLI::is_string_numeric(args[i + 1].clone()) { branch_arg.deepnis = args[i + 1].parse::<u128>().unwrap()}},
                "--show" => branch_arg.hidden = false,
                "-s" => branch_arg.hidden = false,
                "--files" =>    branch_arg.mode = 1,
                "-fi" =>        branch_arg.mode = 1,
                "--folders" =>  branch_arg.mode = 2,
                "-fo" =>        branch_arg.mode = 2,
                _ => ()
            }
        }

        return branch_arg;
    }

    fn print_help() {
        println!("
        
        \tbranch <path> <option> 
        
        \t--show | -s \t\tshow hidden files. (start with '.')
        \t--folders | -fo \tshow only folders.
        \t--files | -fi   \t\tshow only files.
        \t-d <number>\t\tset how many subfolder must be shown.
        \t-p <number>\t\tset the padding to the left side (buggy | not recomended).
        \t--find <file> \t\t scan the directory (futured)"
    );
        exit(0);
    }
    
    fn is_string_numeric(str: String) -> bool {
        for c in str.chars() {
            if !c.is_numeric() {
                return false;
            }
        }
        return true;
    }       
}