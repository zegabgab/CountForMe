fn main() {
    let mut args = main_args::parse_args();
    let mut running = true;
    while running {
        let _ = (args.action)(&mut running);
    }
}

mod main_args {
    type MainAction = dyn FnMut(&mut bool) -> Result<(), ()>;

    pub struct Args {
        pub action: Box<MainAction>,
    }

    pub fn parse_args() -> Args {
        Args {
            action: Box::new(
                |running| count_for_me::process_input::process(&mut std::io::stdin().lock(), running))
        }
    }
}
