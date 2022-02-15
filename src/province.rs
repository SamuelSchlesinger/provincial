use crate::demographics::Demographics;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ProvinceId(u32);

#[derive(Debug, PartialEq)]
pub(crate) struct Province {
    id: ProvinceId,
    name: String,
    description: String,
    population: Demographics,
}
