use crate::{Investigation, MethodToken, ParseResult};
use std::collections::HashMap;

pub trait HasInvestigations {
    fn investigations(&self) -> &Vec<Investigation>;
}

pub trait InvestigationAggregator: HasInvestigations {
    fn count_investigations(&self) -> HashMap<MethodToken, usize> {
        let mut acc = HashMap::new();
        for investigation in self.investigations() {
            if let ParseResult::Parsed(token) = investigation.method.token {
                *acc.entry(token).or_insert(0) += 1;
            }
        }
        acc
    }
}
