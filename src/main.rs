use count_for_me::Shell;

fn main() -> Result<(), String> {
    let shell = Shell::new();
    shell.execute()
}

mod main_args {
    pub struct Args {
        pub interactive: bool,
    }

    impl Args {
        pub fn get() -> Result<Args, &'static str> {
            Ok(Args { interactive: true })
        }
    }
}
