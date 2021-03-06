use crate::grammar::Grammar;
use crate::rule::{CompiledRule, RegExpSourceList, Rule, BeginEndRule, BeginWhileRule, CaptureRule, MatchRule, EmptyRule, IncludeOnlyRule};
use core::fmt;
use dyn_clone::{clone_trait_object, DynClone};
use scie_scanner::scanner::scanner::IOnigCaptureIndex;
use crate::support::regex_source::RegexSource;

pub enum RuleEnum {
    BeginEndRule(BeginEndRule),
    BeginWhileRule(BeginWhileRule),
    CaptureRule(CaptureRule),
    MatchRule(MatchRule),
    EmptyRule(EmptyRule),
    IncludeOnlyRule(IncludeOnlyRule),
}

pub trait AbstractRule: DynClone + erased_serde::Serialize {
    fn id(&self) -> i32;
    fn type_of(&self) -> String;
    fn display(&self) -> String {
        String::from("AbstractRule")
    }
    // todo: add support for this;
    fn get_rule(&self) -> Rule;
    fn get_rule_instance(&self) -> RuleEnum;
    fn get_name(&self, line_text: Option<String>, capture_indices: Option<Vec<IOnigCaptureIndex>>) -> Option<String> {
        let name = self.get_rule()._name.clone();
        let has_captures = RegexSource::has_captures(name.clone());
        if let None = capture_indices {
            return name
        }
        if !has_captures || name == None || line_text == None {
            return name
        }

        return Some(String::from(""));
    }
    fn has_missing_pattern(&self) -> bool {
        false
    }
    fn patterns_length(&self) -> i32 {
        -1
    }
    fn collect_patterns_recursive(
        &mut self,
        grammar: &mut Grammar,
        out: &mut RegExpSourceList,
        is_first: bool,
    );
    fn compile(
        &mut self,
        grammar: &mut Grammar,
        end_regex_source: Option<String>,
        allow_a: bool,
        allow_g: bool,
    ) -> CompiledRule;
}

impl fmt::Debug for dyn AbstractRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

serialize_trait_object!(AbstractRule);

clone_trait_object!(AbstractRule);
