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

table! {
    languages (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    snippets (id) {
        id -> Int4,
        link -> Varchar,
        code -> Text,
    }
}

table! {
    snippets_languages (id) {
        id -> Int4,
        snippet_id -> Nullable<Int4>,
        language_id -> Nullable<Int4>,
    }
}

table! {
    snippets_tags (id) {
        id -> Int4,
        snippet_id -> Nullable<Int4>,
        tag_id -> Nullable<Int4>,
    }
}

table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(snippets_languages -> languages (language_id));
joinable!(snippets_languages -> snippets (snippet_id));
joinable!(snippets_tags -> snippets (snippet_id));
joinable!(snippets_tags -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    languages,
    snippets,
    snippets_languages,
    snippets_tags,
    tags,
);
