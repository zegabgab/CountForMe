use std::io::{stdin, BufRead};

pub mod process_input;

pub enum ResponseType {
    Terminate,
    Continue,
}

pub struct Response {
    kind: ResponseType,
}

impl Response {
    pub fn continuing() -> Response {
        Response {
            kind: ResponseType::Continue,
        }
    }

    pub fn terminating() -> Response {
        Response {
            kind: ResponseType::Terminate,
        }
    }
}

pub struct Shell<F>
where
    F: FnMut(&str) -> Response,
{
    function: F,
}

impl<F> Shell<F>
where
    F: FnMut(&str) -> Response,
{
    pub fn new(function: F) -> Shell<F> {
        Shell { function }
    }

    pub fn process(&mut self, line: &str) -> Response {
        (self.function)(line)
    }

    pub fn execute(mut self) -> Result<(), String> {
        stdin()
            .lock()
            .lines()
            .map(|line| {
                line.map(|line| self.process(&line))
                    .map_err(|err| format!("Error reading line: {}", err.kind()))
            })
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
