use std::iter::zip;

use crate::SyntaxTree;

pub fn parse_term(input: &Vec<String>) -> Option<SyntaxTree> {
    let rules = [
        GrammarRule {
            name: String::from("Var"),
            production: Production { components: vec![String::from("banana")] }
        },
        GrammarRule {
            name: String::from("Term"),
            production: Production { components: vec![String::from("Var")] }
        },
        GrammarRule {
            name: String::from("Term"),
            production: Production { components: vec![String::from("("), String::from("Term"), String::from(")")] }
        },
        GrammarRule {
            name: String::from("Term"),
            production: Production { components: vec![String::from("Term"), String::from("+"), String::from("Term")] }
        }
    ];
    let mut syntaxed_input: (Vec<SyntaxTree>, bool) = (input.iter().map(|s| SyntaxTree::new(s)).collect(), true);

    loop {
        syntaxed_input = parse_partial(syntaxed_input.0, &rules);
        if !syntaxed_input.1 {
            break None;
        } else if syntaxed_input.0.len() == 1 && syntaxed_input.0[0].name() == "Term"{
            break Some(syntaxed_input.0.remove(0));
        }
    }
}

fn parse_partial(mut input: Vec<SyntaxTree>, rules: &[GrammarRule]) -> (Vec<SyntaxTree>, bool) {
    let mut result = Vec::new();
    let iter = input.iter();
    let iter_2 = iter.clone();
    let mut progress = false;
    while input.len() > 0 {
        let mut match_found = false;
        for rule in rules {
            match_found = true;
            if rule.production.components.len() <= input.len() {
                for i in 0..rule.production.components.len() {
                    if rule.production.components[i] != input[i].name() {
                        match_found = false;
                    }
                }
            } else {
                match_found = false;
            }
            if match_found {
                progress = true;
                let mut components = Vec::new();
                for _ in 0..rule.production.components.len() {
                    components.push(input.remove(0));
                }
                result.push(SyntaxTree::with_children(&rule.name, components));
                break;
            }
        }
        if !match_found {
            result.push(input.remove(0));
        }
    }
(result, progress)
}

pub struct GrammarRule {
    name: String,
    production: Production
}

pub struct Production {
    components: Vec<String>
}