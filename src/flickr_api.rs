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
        Err(e) => Err(format!("unable to get flickr licenses, {}, {:?}", e.description(), e.cause()))
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Owner {
    pub username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PhotoInfo {
    pub id: String,
    pub originalsecret: String,
    pub license: i32,
    pub owner: Owner,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetInfoPhotoPayload {
    pub photo: PhotoInfo,
}

pub fn get_photo_info(key: &str, photo_id: &str) -> Result<PhotoInfo, String> {
    info!("calling flickr.photos.getInfo api");
    let url = format!("https://api.flickr.com/services/rest/?method=flickr.photos.getInfo&api_key={}&photo_id={}&format=json&nojsoncallback=1", key, photo_id);

    match Client::new().unwrap().get(&*url).send() {
        Ok(mut r) => {
            if r.status().is_success() {
                let mut buf = String::new();
                match r.read_to_string(&mut buf) {
                    Ok(_) => {
                        info!("body: {}", buf);
                        // unserialize
                        match serde_json::from_str::<GetInfoPhotoPayload>(&*buf) {
                            Ok(v) => Ok(v.photo),
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
        Err(e) => Err(format!("unable to get flickr photo info, {}", e.description()))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Photo {
    pub id: String,
    pub secret: String,
    pub server: String,
    pub farm: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Photos {
    pub photo: Vec<Photo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchPhotosPayload {
    pub photos: Photos,
}

pub fn search_photos(key: &str, search_str: String, place_id: Option<String>)
                     -> Result<Vec<Photo>, String> {
    info!("calling flickr.photos.search api");
    let search_str = search_str.replace(" ", "+");
    let with_place_id = match place_id {
        Some(pid) => _search_photos(key, &*search_str, &*pid),
        None => Ok(vec![]),
    };
    let without_place_id = _search_photos(key, &*search_str, "");
    resolve_smallest_photos(with_place_id, without_place_id)
}

fn resolve_smallest_photos(p1: Result<Vec<Photo>, String>, p2: Result<Vec<Photo>, String>)
                           -> Result<Vec<Photo>, String> {
    if p1.is_err() {
        p2
    } else if p2.is_err() {
        p1
    } else {
        let p1_content = p1.unwrap();
        let p2_content = p2.unwrap();
        Ok(match (p1_content.len(), p2_content.len()) {
            (0, _) => p1_content,
            (_, 0) => p2_content,
            (i, j) if i < j => p1_content,
            (i, j) if j < i => p2_content,
            (_, _) => p1_content,
        })
    }
}

fn _search_photos(key: &str, search_str: &str, place_id: &str) -> Result<Vec<Photo>, String> {
    let url = format!("https://api.flickr.com/services/rest/?method=flickr.photos.search&per_page=10&api_key={}&text={}&license=1%2C2%2C3%2C4%2C5%2C6%2C7%2C9%2C10&place_id={}&format=json&nojsoncallback=1", key, search_str, place_id);

    match Client::new().unwrap().get(&*url).send() {
        Ok(mut r) => {
            if r.status().is_success() {
                let mut buf = String::new();
                match r.read_to_string(&mut buf) {
                    Ok(_) => {
                        // unserialize
                        match serde_json::from_str::<SearchPhotosPayload>(&*buf) {
                            Ok(v) => Ok(v.photos.photo),
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
        Err(e) => Err(format!("unable to get flickr photos, {}", e.description()))
    }

}
