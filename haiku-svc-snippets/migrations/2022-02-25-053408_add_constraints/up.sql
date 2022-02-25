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

-- Your SQL goes here

ALTER TABLE snippets_tags
      add constraint fk_snippets_tags
      foreign key (snippet_id)
      REFERENCES snippets (id) ON UPDATE CASCADE ON DELETE CASCADE;

ALTER TABLE snippets_tags
      add constraint fk_tags_snippets
      foreign key (tag_id)
      REFERENCES tags (id) ON UPDATE CASCADE ON DELETE CASCADE;

ALTER TABLE snippets_languages
      add constraint fk_snippets_languages
      foreign key (snippet_id)
      REFERENCES snippets (id) ON UPDATE CASCADE ON DELETE CASCADE;

ALTER TABLE snippets_languages
      add constraint fk_languages_snippets
      foreign key (language_id)
      REFERENCES languages (id) ON UPDATE CASCADE ON DELETE CASCADE;
