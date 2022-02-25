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
