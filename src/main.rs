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
extern crate xml;

use clap::{App, Arg};
use monument::Monument;
use xml::reader::{XmlEvent, EventReader};

mod monument;
mod unesco_xml;

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

fn read_xml(xml: &str) -> Vec<Monument> {
    let parser = EventReader::new(xml.as_bytes());
    let mut current_monument = Monument::new();
    let mut monuments = vec![];
    let mut in_row = false;
    let mut current_element = String::new();

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if !in_row && &*name.local_name == "row".to_string() {
                    in_row = true;
                } else if in_row {
                    current_element = name.local_name.clone();
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                if in_row && &*name.local_name == "row".to_string() {
                    in_row = false;
                    monuments.push(current_monument);
                    current_monument = Monument::new();
                    current_element = String::new();
                }
            }
            Ok(XmlEvent::Characters(s))=> {
                current_monument.set(&*current_element, &*s);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {/* ignore rest */}
        }
    }

    return monuments;
}

fn main() {
    let _ = env_logger::init();
    let args = parse_cmdline();
    let whl_payload = match unesco_xml::get(args.xml) {
        Ok(p) => p,
        Err(e) => {
            error!("{}", e);
            return
        }
    };
    println!("payload size: {}", whl_payload.len());
    let monuments = read_xml(&*whl_payload);
    for m in monuments {
        println!("{:?}", m);
    }
}
