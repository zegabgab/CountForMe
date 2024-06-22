pub mod process_input;

pub enum ResponseType {
    Terminate,
    Continue,
}

pub struct Response {
    kind: ResponseType
}

pub struct Shell;

impl Shell {
    pub fn process(&mut self, _: &[u8]) -> Response {
        todo!()
    }
    
    pub fn execute(self) -> Result<(), &'static str> {
        todo!()
    }
}