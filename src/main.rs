// Copyright 2017 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(proc_macro)]

//extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate clap;
extern crate env_logger;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;
extern crate xml;

use clap::{App, Arg};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::migrations;
use domain::{Monument, License};
use std::io::stdout;
use std::path::Path;
use uuid::Uuid;
use xml::reader::{XmlEvent, EventReader};

mod domain;
mod flickr_api;
mod unesco_xml;

struct CmdLineArgs {
    pub pq_addr: String,
    pub migrations: Option<String>,
    pub xml: Option<String>,
    pub flickr_key: Option<String>,
}

fn parse_cmdline() -> CmdLineArgs {
    let matches = App::new("whlp")
        .version("v0.1.0")
        .global_setting(clap::AppSettings::ColoredHelp)
        .about("retrieve information from unesco whc and store them in pq")
        .arg(Arg::with_name("pq-addr")
             .long("pq-addr")
             .help("postgres server address")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("migrations")
             .long("migrations")
             .help("database migrations folder")
             .takes_value(true))
        .arg(Arg::with_name("xml")
             .long("xml")
             .help("use local whc xml file")
             .takes_value(true))
        .arg(Arg::with_name("flickr-key")
             .long("flickr-key")
             .help("flicker api key to list pictures of the monuments")
             .takes_value(true))
        .get_matches();

    CmdLineArgs {
        pq_addr: matches.value_of("pq-addr").unwrap().into(),
        migrations: matches.value_of("migrations").map_or(None, |s| Some(s.into())),
        xml: matches.value_of("xml").map_or(None, |s| Some(s.into())),
        flickr_key: matches.value_of("flickr-key").map_or(None, |s| Some(s.into())),
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

pub fn establish_connection(pq_addr: &str) -> PgConnection {
    PgConnection::establish(pq_addr)
        .expect(&format!("Error connecting to {}", pq_addr))
}

pub fn run_migrations(conn: &PgConnection, migrations_path: Option<String>) {
    // if migrations are specified by the user, just run then
    // or do nothing
    match migrations_path {
        Some(m) => {
            info!("try to find migrations in path: {}", m);
            match migrations::search_for_migrations_directory(Path::new(&*m)) {
                Ok(pb) => {
                    info!("migrations found at: {}", pb.to_str().unwrap());
                    info!("executing migrations ...");
                    match migrations::run_pending_migrations_in_directory(conn, pb.as_path(), &mut stdout()) {
                        Ok(_) => info!("migrations executed with success !"),
                        Err(e) => panic!(format!("{}", e)),
                    }
                },
                Err(e) => panic!(format!("{}", e)),
            }
        },
        None => {}
    }
}

pub fn insert_monuments(conn: &PgConnection, monuments: &mut Vec<Monument>) {
    use domain::schema::monuments;

    for m in monuments.iter_mut() {
        m.id = Uuid::new_v4().to_string();
        diesel::insert(m).into(monuments::table)
            .execute(conn)
            .expect("Error saving new monument");
        debug!("new monument added: {:?}", m);
    }
    info!("{} monuments successfully saved", monuments.len());
}

pub fn insert_licenses(conn: &PgConnection, key: &str) {
    use domain::schema::licenses;

    let licenses = match flickr_api::get_licenses(key) {
        Ok(ls) => ls,
        Err(e) => panic!(format!("{}", e))
    };

    for rl in licenses.clone() {
        let mut l: License = rl.into();
        l.id = Uuid::new_v4().to_string();
        diesel::insert(&l).into(licenses::table)
            .execute(conn)
            .expect("Error saving new licenses");
        debug!("new lisense added: {:?}", l);
    }

    info!("{} licenses successfully saved", licenses.len());
}

pub fn insert_pictures(conn: &PgConnection, monuments: &Vec<Monument>, key: &str) {

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
    // get pq connection
    let conn = establish_connection(&*args.pq_addr);

    // run migration if needed
    run_migrations(&conn, args.migrations);


    // first insert monuments
    let mut monuments = read_xml(&*whl_payload);
    insert_monuments(&conn, &mut monuments);

    // then if api key for flickr is used, get picture from flickr
    match args.flickr_key {
        Some(key) => {
            insert_licenses(&conn, &key);
            insert_pictures(&conn, &monuments, &key);
        },
        None => {},
    }
}
