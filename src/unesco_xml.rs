// Copyright 2017 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use hyper::{self, Client};
use std::error::Error;
use std::fs::File;
use std::io::Read;

const UNESCO_XML: &'static str = "http://whc.unesco.org/en/list/xml/";

pub fn get(file: Option<String>) -> Result<String, String> {
    match file {
        Some(f) => from_file(f),
        None => from_download(),
    }
}

pub fn from_file(file: String) -> Result<String, String> {
    info!("reading unesco xml: '{}'", file);
    match File::open(file) {
        Ok(mut f) => {
            let mut buf = String::new();
            match f.read_to_string(&mut buf) {
                Ok(_) => Ok(buf),
                Err(e) => Err(format!("unable to read file, {}", e))
            }
        },
        Err(e) => Err(format!("unable to open file: {}", e.description())),
    }
}

pub fn from_download() -> Result<String, String> {
    info!("downloading unesco xml");
    match Client::new().get(UNESCO_XML).send() {
        Ok(mut r) => {
            if r.status == hyper::Ok {
                let mut buf = String::new();
                match r.read_to_string(&mut buf) {
                    Ok(_) => Ok(buf),
                    Err(e) => Err(format!("unable to read http request payload, try again, {}", e))
                }
            } else {
                Err(format!("unexpected http status, try again"))
            }
        },
        Err(e) => Err(format!("unable to get whc xml, {}", e.description()))
    }
}
