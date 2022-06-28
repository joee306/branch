mod branch;
mod cli;
mod config;
use branch::{Branch, Show_mode};


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
    

    let config = config::new();

    } else {
        let attributes : cli::Branch_arg = cli::CLI::handle_args(&args);
        Branch::display(
            attributes.path,
            attributes.padding,
            attributes.deepnis,
            match attributes.mode {
                1 => &Show_mode::FILES,
                2 => &Show_mode::FOLDER,
                _ => &Show_mode::ALL
            },
            attributes.hidden
        );
    }
}
