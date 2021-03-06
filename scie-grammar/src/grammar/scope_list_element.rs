use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use crate::grammar::Grammar;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeListElement {
    pub parent: Option<Box<ScopeListElement>>,
    pub scope: String,
    // pub metadata: i32,
}

impl ScopeListElement {
    pub fn new(parent: Option<Box<ScopeListElement>>, scope: String) -> Self {
        ScopeListElement {
            parent,
            scope,
            // metadata,
        }
    }

    pub fn generate_scopes(&self) -> Vec<String> {
        let mut result = vec![];

        let mut scope_list = self.clone();
        let mut is_scope_list_none = false;
        while !is_scope_list_none  {
            result.push(scope_list.scope.clone());
            match scope_list.parent {
                None => {
                    is_scope_list_none = true
                },
                Some(scope_value) => {
                    scope_list = *scope_value.clone();
                },
            }
        }

        result.reverse();
        return result.clone();
    }

    pub fn _push(origin_target: ScopeListElement, grammar: &mut Grammar, scopes: Vec<String>) -> ScopeListElement {
        let mut target = origin_target.clone();
        for scope in scopes {
            target = ScopeListElement::new(Some(Box::new(target)), scope);
        }
        target
    }
    pub fn push(&self, grammar: &mut Grammar, scope: Option<String>) -> ScopeListElement {
        if let None = scope {
            return self.clone()
        }

        let scope_name = scope.clone().unwrap();
        return match scope.iter().position(|s| s == " ") {
            None => {
                ScopeListElement::_push(self.clone(), grammar, vec![scope_name])
            },
            Some(_) => {
                println!("todo: ScopeListElement push");
                self.clone()
            },
        }

    }
}

impl Default for ScopeListElement {
    fn default() -> Self {
        ScopeListElement {
            parent: None,
            scope: "".to_string(),
            // metadata: 0,
        }
    }
}
