use std::collections::HashSet;

use crate::province::ProvinceId;

pub(crate) struct NationId(u32);

pub(crate) struct Nation {
    name: String,
    description: String,
    provinces: HashSet<ProvinceId>,
}
