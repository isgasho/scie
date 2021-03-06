use crate::grammar::Grammar;
use crate::rule::CompiledRule;

#[derive(Clone, Debug, Serialize)]
pub struct AnchorCache {
    A0_G0: Option<String>,
    A0_G1: Option<String>,
    A1_G0: Option<String>,
    A1_G1: Option<String>,
}

impl Default for AnchorCache {
    fn default() -> Self {
        AnchorCache {
            A0_G0: None,
            A0_G1: None,
            A1_G0: None,
            A1_G1: None,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct RegExpSourceList {
    pub _has_anchors: bool,
    pub _cached: Option<CompiledRule>,
    pub _anchor_cache: AnchorCache,
    pub _items: Vec<Box<RegExpSource>>,
}

impl RegExpSourceList {
    pub fn new() -> Self {
        RegExpSourceList {
            _has_anchors: false,
            _cached: None,
            _anchor_cache: Default::default(),
            _items: vec![],
        }
    }

    pub fn push(&mut self, item: Box<RegExpSource>) {
        self._items.push(item.clone());
        if item.clone().has_anchor {
            self._has_anchors = self._has_anchors || item.has_anchor;
        }
    }

    pub fn unshift(&mut self, item: Box<RegExpSource>) {
        self._items.push(item.clone());
        self._items.rotate_right(1);
        if item.clone().has_anchor {
            self._has_anchors = self._has_anchors || item.has_anchor;
        }
    }

    pub fn compile(&mut self, grammar: &mut Grammar, allow_a: bool, allow_g: bool) -> CompiledRule {
        if !self._has_anchors {
            if let None = self._cached {
                let reg_exps = self
                    ._items
                    .clone()
                    .into_iter()
                    .map(|x| x.source)
                    .collect::<Vec<String>>();
                let rules = self
                    ._items
                    .clone()
                    .into_iter()
                    .map(|x| x.rule_id)
                    .collect::<Vec<i32>>();
                let compiled_rule = CompiledRule::new(reg_exps, rules);
                self._cached = Some(compiled_rule.clone());
                return compiled_rule;
            }
        } else {
            println!("// todo: cached {:?}", self._items);
        }

        CompiledRule::new(vec![], vec![])
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct RegExpSource {
    pub source: String,
    pub rule_id: i32,
    pub has_anchor: bool,
}

impl RegExpSource {
    pub fn new(reg_exp_source: String, rule_id: i32) -> RegExpSource {
        RegExpSource {
            source: reg_exp_source,
            rule_id,
            has_anchor: false,
        }
    }
}
