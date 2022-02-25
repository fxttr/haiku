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

use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::result::Error;
use r2d2::PooledConnection;
use crate::data::query::snippet::Snippet;
use crate::data::traits::base_query::BaseQuery;

pub struct SnippetsController {
    connection: PooledConnection<ConnectionManager<PgConnection>>
}

impl SnippetsController {
    pub fn new(connection: PooledConnection<ConnectionManager<PgConnection>>) -> Self {
        SnippetsController{
            connection
        }
    }

    pub fn find(&self) -> Result<Vec<Snippet>, Error> {
        Snippet::find(&self.connection)
    }

    pub fn find_by_id(&self, id: i32) -> Result<Snippet, Error> {
        Snippet::find_by_id(&self.connection, id)
    }
}