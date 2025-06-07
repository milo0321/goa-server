use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PackingField {
    pub value: OrderedFloat<f32>,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SizeField {
    pub length: OrderedFloat<f32>,
    pub width: OrderedFloat<f32>,
    pub height: OrderedFloat<f32>,
    pub unit: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct PackingDetail {
    pub bag_pack: PackingField,
    pub carton_pack: PackingField,
    pub carton_size: SizeField,
    pub weight: PackingField,
}
