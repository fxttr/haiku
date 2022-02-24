-- Your SQL goes here
CREATE TABLE snippets (
       id SERIAL PRIMARY KEY,
       link varchar(100) NOT NULL,
       code TEXT NOT NULL,
       constraint fk_snippets_languages_cross
       		  foreign key (id)
		  REFERENCES snippets_languages (snippet_id),
	constraint fk_snippets_tags_cross
       		  foreign key (id)
		  REFERENCES snippets_tags (snippet_id)
)
