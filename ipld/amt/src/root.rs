// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

use crate::Node;
use encoding::{
    de::{self, Deserialize},
    ser, Cbor,
};

#[derive(PartialEq, Eq, Debug, Default)]
pub struct Root<'a> {
    pub(super) height: u64,
    pub(super) count: u64,
    pub(super) node: Node<'a>,
}

impl ser::Serialize for Root<'_> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        (self.height, self.count, self.node.clone()).serialize(s)
    }
}

impl<'de> de::Deserialize<'de> for Root<'_> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let (height, count, node) = Deserialize::deserialize(deserializer)?;
        Ok(Self {
            height,
            count,
            node,
        })
    }
}

impl Cbor for Root<'_> {}