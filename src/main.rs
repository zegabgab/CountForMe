mod syntax_tree;
mod parse;
mod process_input;

use syntax_tree::SyntaxTree;

fn main() {
    let mut args = main_args::parse_args();
    let mut running = true;
    while running {
        let _ = process_input::process(args.reader(), &mut running);
    }
}

mod main_args {
    pub struct Args {
        reader: Box<dyn std::io::BufRead>
    }

    pub fn parse_args() -> Args {
        Args {
            reader: Box::new(std::io::stdin().lock())
        }
    }

    impl Args {
        pub fn reader(&mut self) -> &mut impl std::io::BufRead {
            &mut self.reader
        }
    }
}