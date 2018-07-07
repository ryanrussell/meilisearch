use std::cmp::Ordering;
use Match;
use rank::{match_query_index, Document};
use group_by::GroupBy;

pub fn sum_of_typos(lhs: &Document, rhs: &Document) -> Ordering {
    let key = |matches: &[Match]| -> u8 {
        GroupBy::new(matches, match_query_index).map(|m| m[0].distance).sum()
    };

    key(&lhs.matches).cmp(&key(&rhs.matches))
}
