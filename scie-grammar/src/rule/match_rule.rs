use crate::grammar::Grammar;
use crate::inter::ILocation;
use crate::rule::{AbstractRule, CompiledRule, Rule};
use crate::rule::{RegExpSource, RegExpSourceList};
use crate::rule::abstract_rule::RuleEnum;

#[derive(Clone, Debug, Serialize)]
pub struct MatchRule {
    pub rule: Rule,
    pub _match: RegExpSource,
    pub captures: Vec<Box<dyn AbstractRule>>,
    pub _cached_compiled_patterns: Option<RegExpSourceList>,
}

impl MatchRule {
    pub fn new(
        location: Option<ILocation>,
        id: i32,
        name: Option<String>,
        _match: String,
        captures: Vec<Box<dyn AbstractRule>>,
    ) -> Self {
        MatchRule {
            rule: Rule {
                _type: String::from("MatchRule"),
                _location: location,
                id,
                _name: name,
                _content_name: None,
            },
            _match: RegExpSource::new(_match, id),
            captures,
            _cached_compiled_patterns: None,
        }
    }
}

impl AbstractRule for MatchRule {
    fn id(&self) -> i32 {
        self.rule.id
    }
    fn type_of(&self) -> String {
        String::from(self.rule.clone()._type)
    }
    fn display(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    fn get_rule(&self) -> Rule {
        self.rule.clone()
    }
    fn get_rule_instance(&self) -> RuleEnum {
        RuleEnum::MatchRule(self.clone())
    }
    fn collect_patterns_recursive(
        &mut self,
        grammar: &mut Grammar,
        out: &mut RegExpSourceList,
        is_first: bool,
    ) {
        out.push(Box::new(self._match.clone()));
    }

    fn compile(
        &mut self,
        grammar: &mut Grammar,
        end_regex_source: Option<String>,
        allow_a: bool,
        allow_g: bool,
    ) -> CompiledRule {
        let mut cached_compiled_patterns = RegExpSourceList::new();

        if let None = self._cached_compiled_patterns {
            self.collect_patterns_recursive(grammar, &mut cached_compiled_patterns, true);
            self._cached_compiled_patterns = Some(cached_compiled_patterns.clone());
        } else {
            cached_compiled_patterns = self._cached_compiled_patterns.as_ref().unwrap().clone();
        }

        return cached_compiled_patterns
            .compile(grammar, allow_a, allow_g)
            .clone();
    }
}
