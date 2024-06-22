use std::io::{stdin, BufRead, Read};

pub mod process_input;

pub enum ResponseType {
    Terminate,
    Continue,
}

pub struct Response {
    kind: ResponseType,
}

pub struct Shell;

impl Shell {
    pub fn new() -> Shell {
        Shell {}
    }

    pub fn process(&mut self, line: &str) -> Response {
        println!("{}", line.to_ascii_uppercase());
        Response { kind: if line == "quit" { ResponseType::Terminate } else { ResponseType::Continue} }
    }

    pub fn execute(mut self) -> Result<(), String> {
        stdin().lock().lines()
        .map(|line| line
            .map(|line| {
                self.process(&line)
            })
            .map_err(|err| format!("Error reading line: {}", err.kind())))
        .take_while(|result| match result {
            Ok(response) => match response.kind {
                ResponseType::Terminate => false,
                ResponseType::Continue => true,
            },
            Err(_) => true,
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|_| ())
    }
}
