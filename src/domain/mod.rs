// Copyright 2017 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use chrono;
use chrono::offset::utc::UTC;
use flickr_api::License as RawLicense;
use self::schema::{
    licenses,
    monuments,
    pictures
};
use std::str::FromStr;
use uuid::Uuid;

pub mod schema;
pub mod dao;

#[derive(Clone, PartialEq, Debug, Queryable, Insertable)]
#[table_name="monuments"]
pub struct Monument {
    pub id: String,
    pub category: Option<String>,
    pub criteria_txt: Option<String>,
    pub danger: Option<String>,
    pub date_inscribed: Option<String>,
    pub extension: Option<i32>,
    pub historical_description: Option<String>,
    pub http_url: Option<String>,
    pub id_number: Option<i32>,
    pub image_url: Option<String>,
    pub iso_code: Option<String>,
    pub justification: Option<String>,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub location: Option<String>,
    pub long_description: Option<String>,
    pub region: Option<String>,
    pub revision: Option<i32>,
    pub secondary_dates: Option<String>,
    pub short_description: Option<String>,
    pub site: Option<String>,
    pub states: Option<String>,
    pub transboundary: Option<i32>,
    pub unique_number: Option<i32>,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Monument {
    pub fn new() -> Monument {
        let now = chrono::NaiveDateTime::from_timestamp(UTC::now().timestamp(), 0);
        Monument {
            id: String::new(),
            category: None,
            criteria_txt: None,
            danger: None,
            date_inscribed: None,
            extension: None,
            historical_description: None,
            http_url: None,
            id_number: None,
            image_url: None,
            iso_code: None,
            justification: None,
            latitude: None,
            longitude: None,
            location: None,
            long_description: None,
            region: None,
            revision: None,
            secondary_dates: None,
            short_description: None,
            site: None,
            states: None,
            transboundary: None,
            unique_number: None,

            created_at: now.clone(),
            updated_at: now
        }
    }

    pub fn set(&mut self, field: &str, value: &str) {
        match field {
            "category" => self.category = FromStr::from_str(value).ok(),
            "criteria_txt" => self.criteria_txt = FromStr::from_str(value).ok(),
            "danger" => self.danger = FromStr::from_str(value).ok(),
            "date_inscribed" => self.date_inscribed = FromStr::from_str(value).ok(),
            "extension" => self.extension = FromStr::from_str(value).ok(),
            "historical_description" => self.historical_description = FromStr::from_str(value).ok(),
            "http_url" => self.http_url = FromStr::from_str(value).ok(),
            "id_number" => self.id_number = FromStr::from_str(value).ok(),
            "image_url" => self.image_url = FromStr::from_str(value).ok(),
            "iso_code" => self.iso_code = FromStr::from_str(value).ok(),
            "justification" => self.justification = FromStr::from_str(value).ok(),
            "latitude" => self.latitude = FromStr::from_str(value).ok(),
            "longitude" => self.longitude = FromStr::from_str(value).ok(),
            "location" => self.location = FromStr::from_str(value).ok(),
            "long_description" => self.long_description = FromStr::from_str(value).ok(),
            "region" => self.region = FromStr::from_str(value).ok(),
            "revision" => self.revision = FromStr::from_str(value).ok(),
            "secondary_dates" => self.secondary_dates = FromStr::from_str(value).ok(),
            "short_description" => self.short_description = FromStr::from_str(value).ok(),
            "site" => self.site = FromStr::from_str(value).ok(),
            "states" => self.states = FromStr::from_str(value).ok(),
            "transboundary" => self.transboundary = FromStr::from_str(value).ok(),
            "unique_number" => self.unique_number = FromStr::from_str(value).ok(),
            _ => {/* unknown name */}
        }
    }
}

#[derive(Clone, PartialEq, Debug, Queryable, Insertable)]
#[table_name="licenses"]
pub struct License {
    pub id: String,
    pub flickr_id: i32,
    pub name: String,
    pub url: Option<String>,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<RawLicense> for License {
    fn from(rl: RawLicense) -> Self {
        let now = chrono::NaiveDateTime::from_timestamp(UTC::now().timestamp(), 0);
        License {
            id: String::new(),
            flickr_id: rl.id,
            name: rl.name,
            url: rl.url,
            created_at: now.clone(),
            updated_at: now
        }
    }
}

#[derive(Clone, PartialEq, Debug, Queryable, Insertable)]
#[table_name="pictures"]
pub struct Picture {
    pub id: String,
    pub flickr_id: String,
    pub monument_id: String,
    pub license_id: String,
    pub author: String,
    pub url: String,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Picture {
    pub fn new(flickr_id: String,
               monument_id: String,
               license_id: String,
               author: String,
               url: String) -> Picture {
        let now = chrono::NaiveDateTime::from_timestamp(UTC::now().timestamp(), 0);
        Picture {
            id: Uuid::new_v4().to_string(),
            flickr_id: flickr_id,
            monument_id: monument_id,
            license_id: license_id,
            author: author,
            url: url,

            created_at: now.clone(),
            updated_at: now
        }
    }
}
