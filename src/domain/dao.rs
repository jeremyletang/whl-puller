// Copyright 2017 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use diesel;
use diesel::pg::PgConnection;
use super::{Picture, Monument, License, LastUpdate};

pub fn update_monument(conn: &PgConnection, m: &Monument) {
    use diesel::{ExecuteDsl, FilterDsl, ExpressionMethods};
    use domain::schema::monuments::dsl::{monuments, id, site, long_description};
    let _ = diesel::update(monuments.filter(id.eq(&m.id)))
        .set(site.eq(&m.site)).execute(conn);
    let _ = diesel::update(monuments.filter(id.eq(&m.id)))
        .set(long_description.eq(&m.long_description)).execute(conn);
}

pub fn update_last_update(conn: &PgConnection, u: &LastUpdate) {
    use diesel::{ExecuteDsl, FilterDsl, ExpressionMethods};
    use domain::schema::last_updates::dsl::{last_updates, id, updated_at};
    let _ = diesel::update(last_updates.filter(id.eq(&u.id)))
        .set(updated_at.eq(u.updated_at)).execute(conn);
}

pub fn last_update_by_monument_id(conn: &PgConnection, mid: &str) -> Option<LastUpdate> {
    use diesel::{LoadDsl, FilterDsl, ExpressionMethods};
    use domain::schema::last_updates::dsl::{last_updates, monument_id};
    last_updates.filter(monument_id.eq(mid)).first::<LastUpdate>(conn).ok()
}

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
