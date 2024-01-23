pub fn parse_term(input: &str) -> Option<SyntaxTree> {
    SyntaxTree::new("term")
}

pub fn parse(input: &str, rule: GrammarRule) -> Option<SyntaxTree> {

}

pub struct GrammarRule {
    name: String,
    production: Production
}

pub struct Production {
    components: Vec<String>
}