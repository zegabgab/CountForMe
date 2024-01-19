pub struct ListLinked<T> {
    start: Option<LinkedNode<T>>
}

struct LinkedNode<T> {
    value: T,
    next: Option<Box<LinkedNode<T>>>
}

impl<T> ListLinked<T> {
    pub fn new() -> ListLinked<T> {
        ListLinked {
            start: None
        }
    }
    
    pub fn get(&self, index: u32) -> Result<&T, String> {
        match &self.start {
            None => return Err(format!("Index {index} out of bounds!")),
            Some(node) => node.get(index, 0)
        }
    }

    pub fn add(&mut self, value: T, index: u32) {
        match &mut self.start {
            None => {self.start = Some(LinkedNode {next: None, value}); }
            Some(node) => {node.add(value, index, 0); }
        }
    }
}

impl<T> LinkedNode<T> {
    fn get(&self, index: u32, current: u32) -> Result<&T, String> {
        match index - current {
            0 => Ok(&self.value),
            _ => match &self.next {
                None => Err(format!("Index {index} out of bounds!")),
                Some(t) => t.get(index, current + 1)
            }
        }
    }

    fn add(&mut self, mut value: T, index: u32, current: u32) {
        match index - current {
            0 => {
                let mut next = None;
                std::mem::swap(&mut self.next, &mut next);
                std::mem::swap(&mut value, &mut self.value);
                self.next = Some(Box::new(LinkedNode {value, next}));
            }
            _ => match &mut self.next {
                None => self.next = Some(Box::new(LinkedNode {value, next: None})),
                Some(node) => node.add(value, index, current + 1)
            }
        }
    }
}