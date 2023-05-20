//! GRIB2 declaration

pub mod bit_map_utils_impl;
pub mod decode_utils_impl;
pub mod section;
pub mod type_utils_impl;
pub mod utils_impl;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use self::section::SectionSets;
use self::utils_impl::parse;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParameterDescription {
    pub(crate) name: String,
    pub(crate) unit: String,
}

#[derive(Clone)]
pub struct Grib2<'a> {
    // key = discipline, parameter_category, parameter_number
    parameter_descriptions:
        Option<BTreeMap<usize, BTreeMap<usize, BTreeMap<usize, ParameterDescription>>>>,

    sectionsets: SectionSets<'a>,
}

impl<'a> Grib2<'a> {
    pub fn new() -> Self {
        let json = include_str!("parameter-descriptions.json");
        let parameter_descriptions = match serde_json::from_str(json) {
            Ok(s) => Some(s),
            Err(msg) => {
                eprintln!("Failed load paramter.json: {}", msg);
                None
            }
        };

        Self {
            parameter_descriptions,
            sectionsets: SectionSets::new(),
        }
    }

    pub fn parse(&mut self, buf: &'a [u8]) {
        self.sectionsets.extend(parse(&buf));
    }

    pub fn sectionsets(&self) -> &SectionSets<'a> {
        return &self.sectionsets;
    }

    pub fn parameter_description(
        &self,
        discipline: usize,
        parameter_category: usize,
        parameter_number: usize,
    ) -> Option<&ParameterDescription> {
        return Some(
            self.parameter_descriptions
                .as_ref()?
                .get(&discipline)?
                .get(&parameter_category)?
                .get(&parameter_number)?,
        );
    }

    pub fn dump(&self) {
        for (i, sectionset) in self.sectionsets.iter().enumerate() {
            println!("{:05}\n{} ", i, sectionset);
        }
    }
}
