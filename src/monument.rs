// Copyright 2017 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct Monument {
    pub category: Option<String>,
    pub criteria: Option<String>,
    pub date_inscribed: Option<String>,
    pub unique_number: Option<i32>,
    pub id_number: Option<i32>,
}

impl Monument {
    pub fn new() -> Monument {
        Monument {
            category: None,
            criteria: None,
            date_inscribed: None,
            unique_number: None,
            id_number: None,
        }
    }

    pub fn set(&mut self, field: &str, value: &str) {
        match field {
            "category" => self.category = Some(value.to_string()),
            "criteria" => self.criteria = Some(value.to_string()),
            "date_inscribed" => self.date_inscribed = Some(value.to_string()),
            "unique_number" => self.unique_number = Some(i32::from_str(value).unwrap()),
            "id_number" => self.id_number = Some(i32::from_str(value).unwrap()),
            _ => {/* unknown name */}
        }
    }
}
