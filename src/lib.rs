pub mod process_input;

pub enum ResponseType {
    Terminate,
    Continue,
}

pub struct Response {
    type: ResponseType
}

pub struct Shell;

impl Shell {
    pub fn process(&mut self, &[u8]) -> Response {
        todo!()
    }
}