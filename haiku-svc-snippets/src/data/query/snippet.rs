/*
 * Haiku
 *
 * Copyright (c) 2022. enso
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::schema::snippets::dsl::snippets;
use crate::data::traits::base_query::BaseQuery;
use diesel::result::Error;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Snippet {
    pub id: i32,
    pub link: String,
    pub code: String,
}

impl BaseQuery for Snippet {
    fn find(connection: &PgConnection) -> Result<Vec<Snippet>, Error> {
        snippets.load::<Snippet>(connection)
    }

    fn find_by_id(connection: &PgConnection, request_id: i32) -> Result<Snippet, Error> {
        use crate::schema::snippets::id;

        snippets
            .filter(id.eq(request_id))
            .first::<Snippet>(connection)
    }
}
