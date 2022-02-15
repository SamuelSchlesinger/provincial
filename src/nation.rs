use std::collections::HashSet;

use crate::province::ProvinceId;

pub(crate) struct Nation {
    name: String,
    description: String,
    provinces: HashSet<ProvinceId>,
}
