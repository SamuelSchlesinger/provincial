use std::collections::BTreeMap;

use crate::nation::{Nation, NationId};
use crate::province::{Province, ProvinceId};

pub(crate) struct Context {
    nations: BTreeMap<NationId, Nation>,
    provinces: BTreeMap<ProvinceId, Province>,
}
