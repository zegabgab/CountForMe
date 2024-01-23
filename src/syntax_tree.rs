pub struct SyntaxTree {
    name: String,
    children: Vec<Self>
}

impl SyntaxTree {
    pub fn new(name: &str) -> SyntaxTree {
        SyntaxTree {
            name: String::from(name),
            children: Vec::new()
        }
    }

    pub fn with_children(name: &str, children: Vec<SyntaxTree>) -> SyntaxTree {
        SyntaxTree {
            name: String::from(name),
            children
        }
    }

    pub fn add_child(&mut self, child: SyntaxTree) {
        self.children.push(child);
    }

    pub fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn child(&self, index: usize) -> &SyntaxTree {
        &self.children[index]
    }

    pub fn child_as_mutable(&mut self, index: usize) -> &mut Self {
        &mut self.children[index]
    }
}

impl std::fmt::Display for SyntaxTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if self.is_leaf() { return Ok(()); }
        write!(f, "[{}", self.child(0))?;
        for child in self.children.iter().skip(1) {
            write!(f, ", {child}")?;
        }
        write!(f, "]")
    }
}