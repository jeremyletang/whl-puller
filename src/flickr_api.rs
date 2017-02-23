// Copyright 2017 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use reqwest::Client;
use serde_json;
use std::error::Error;
use std::io::Read;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct License {
    pub id: i32,
    pub name: String,
    pub url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Licenses {
    pub license: Vec<License>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LicensesPayload {
    pub licenses: Licenses,
}

pub fn get_licenses(key: &str) -> Result<Vec<License>, String> {
    info!("calling flickr.photos.licenses.getInfo api");
    let url = format!("https://api.flickr.com/services/rest/?method=flickr.photos.licenses.getInfo&api_key={}&format=json&nojsoncallback=1", key);
    match Client::new().unwrap().get(&*url).send() {
        Ok(mut r) => {
            if r.status().is_success() {
                let mut buf = String::new();
                match r.read_to_string(&mut buf) {
                    Ok(_) => {
                        // unserialize
                        match serde_json::from_str::<LicensesPayload>(&*buf) {
                            Ok(v) => Ok(v.licenses.license),
                            Err(e) => {
                                Err(format!("unable to deserialize payload, try again, {}", e))
                            }
                        }
                    },
                    Err(e) => Err(format!("unable to read http request payload, try again, {}", e))
                }
            } else {
                Err(format!("unexpected http status, try again"))
            }
        },
        Err(e) => Err(format!("unable to get flickr licenses, {}", e.description()))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Place {
    pub place_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Places {
    pub place: Vec<Place>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FindByLatLonPayload {
    pub places: Places
}

pub enum FindByLatLonError {
    RequestError(String),
    NoMatchingPlace
}

pub fn get_place(key: &str, lat: f32, lng: f32) -> Result<String, FindByLatLonError> {
    info!("calling flickr.places.findByLatLon api");
    let url = format!("https://api.flickr.com/services/rest/?method=flickr.places.findByLatLon&api_key={}&lat={}&lon={}&format=json&nojsoncallback=1", key, lat, lng);
    match Client::new().unwrap().get(&*url).send() {
        Ok(mut r) => {
            if r.status().is_success() {
                let mut buf = String::new();
                match r.read_to_string(&mut buf) {
                    Ok(_) => {
                        match serde_json::from_str::<FindByLatLonPayload>(&*buf) {
                            Ok(v) => {
                                if v.places.place.len() > 0 {
                                    Ok(v.places.place[0].place_id.clone())
                                } else {
                                    Err(FindByLatLonError::NoMatchingPlace)
                                }
                            },
                            Err(e) => Err(FindByLatLonError::RequestError(
                                format!("unable to deserialize payload, try again, {}", e)))
                        }
                    },
                    Err(e) => Err(FindByLatLonError::RequestError(
                        format!("unable to read http request payload, try again, {}", e)))
                }
            } else {
                Err(FindByLatLonError::RequestError(format!("unexpected http status, try again")))
            }
        },
        Err(e) => Err(FindByLatLonError::RequestError(
            format!("unable to get flickr groupd, {}", e.description())))
    }
}
