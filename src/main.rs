// Copyright 2017 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(proc_macro)]

extern crate clap;
extern crate env_logger;
extern crate hyper;
#[macro_use]
extern crate log;
// extern crate serde;
// #[macro_use]
//extern crate serde_derive;
// extern crate serde_xml;

use clap::{App, Arg};
use hyper::Client;
use serde::Deserialize;
use std::error::Error;
use std::io::Read;

const UNESCO_XML: &'static str = "http://whc.unesco.org/en/list/xml/";

// #[derive(PartialEq, Debug, Serialize, Deserialize)]
struct Monument {
    pub category: String,
    pub criteria: String,
    pub date_inscribed: String,
    pub unique_number: i32,
    pub id_number: i32,
}

struct CmdLineArgs {
    pub pq_addr: String,
    pub xml: Option<String>,
}

fn parse_cmdline() -> CmdLineArgs {
    let matches = App::new("arrakis_standalone")
        .version("v0.1.0")
        .global_setting(clap::AppSettings::ColoredHelp)
        .about("retrieve information from unesco whc and store them in pq")
        .arg(Arg::with_name("pq-addr")
             .long("pq-addr")
             .help("postgres server address")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("xml")
             .long("xml")
             .help("use local whc xml file")
             .takes_value(true))
        .get_matches();

    CmdLineArgs {
        pq_addr: matches.value_of("pq-addr").unwrap().into(),
        xml: matches.value_of("xml").map_or(None, |s| Some(s.into())),
    }
}

fn main() {
    let _ = env_logger::init();
    let args = parse_cmdline();
    let whc_payload = match Client::new().get(UNESCO_XML).send() {
        Ok(r) => {
            if r.status == hyper::Ok {
                let mut buf = String::new();
                match r.read_to_string(&mut buf) {
                    Ok(_) => buf,
                    Err(e) => {
                        error!("unable to red http request payload, try again");
                        return
                    }
                }
            } else {
                error!("unexpected http status, try again");
                return
            }
        },
        Err(e) => {
            error!("unable to get whc xml, {}", e.description());
            return
        }
    };

    let whc: Vec<Monument> = match serde_xml::from_str(&*whc_payload) {
        Ok(ms) => ms,
        Err(e) => {
            error!("invalid whc xml format, {}", e.description());
            return
        }
    };
}
