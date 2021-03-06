use crate::structures::finder::Opts;
use crate::structures::fnv::HashLine;
use std::collections::HashMap;

pub type Suggestion = (String, Option<Opts>);

#[derive(Clone)]
pub struct VariableMap {
    variables: HashMap<u64, HashMap<String, Suggestion>>,
    dependencies: HashMap<u64, Vec<u64>>,
}

impl VariableMap {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }

    pub fn insert_dependency(&mut self, tags: &str, tags_dependency: &str) {
        let k = tags.hash_line();
        if let Some(v) = self.dependencies.get_mut(&k) {
            v.push(tags_dependency.hash_line());
        } else {
            let mut v: Vec<u64> = Vec::new();
            v.push(tags_dependency.hash_line());
            self.dependencies.insert(k, v);
        }
    }

    pub fn insert_suggestion(&mut self, tags: &str, variable: &str, value: Suggestion) {
        let k1 = tags.hash_line();
        let k2 = String::from(variable);
        if let Some(m) = self.variables.get_mut(&k1) {
            m.insert(k2, value);
        } else {
            let mut m = HashMap::new();
            m.insert(k2, value);
            self.variables.insert(k1, m);
        }
    }

    pub fn get_suggestion(&self, tags: &str, variable: &str) -> Option<&Suggestion> {
        let k = tags.hash_line();
        let res = self.variables.get(&k)?.get(variable);
        if res.is_some() {
            return res;
        }
        if let Some(dependency_keys) = self.dependencies.get(&k) {
            for dependency_key in dependency_keys {
                let res = self.variables.get(&dependency_key)?.get(variable);
                if res.is_some() {
                    return res;
                }
            }
        }
        None
    }
}
