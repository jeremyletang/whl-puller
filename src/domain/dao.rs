// Copyright 2017 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use diesel::pg::PgConnection;
use super::{Picture, Monument, License};

pub fn picture_exists(conn: &PgConnection, fid: &str) -> bool {
    use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
    use domain::schema::pictures::dsl::{pictures, flickr_id};

    match pictures.filter(flickr_id.eq(&fid)).load::<Picture>(conn) {
        Ok(v) => v.len() != 0,
        Err(_) => false,
    }
}

pub fn list_monuments(conn: &PgConnection) -> Vec<Monument> {
    use diesel::{LoadDsl};
    use domain::schema::monuments::dsl::{monuments};
    monuments.load::<Monument>(conn).unwrap()
}

pub fn list_licenses(conn: &PgConnection) -> Vec<License> {
    use diesel::{LoadDsl};
    use domain::schema::licenses::dsl::{licenses};
    licenses.load::<License>(conn).unwrap()
}
