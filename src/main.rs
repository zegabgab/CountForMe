use count_for_me::{Response, Shell};

fn main() -> Result<(), String> {
    let shell = Shell::new(|line| {
        println!("{}", line.to_ascii_uppercase());
        if line == "quit" {
            Response::terminating()
        } else {
            Response::continuing()
        }
    });
    shell.execute()
}
