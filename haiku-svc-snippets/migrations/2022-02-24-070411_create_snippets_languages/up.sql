-- Your SQL goes here
CREATE TABLE snippets_languages (
       id SERIAL PRIMARY KEY,
       snippet_id integer,
       language_id integer,
       constraint fk_snippets_languages
       		  foreign key (snippet_id)
		  REFERENCES snippets (id),
       constraint fk_languages_snippets
       		  foreign key (language_id)
		  REFERENCES languages (id)
)
