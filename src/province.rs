use std::collections::HashSet;

use crate::community::CommunityId;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ProvinceId(u32);

#[derive(Debug, PartialEq)]
pub(crate) struct Province {
    name: String,
    description: String,
    population: HashSet<CommunityId>,
}
