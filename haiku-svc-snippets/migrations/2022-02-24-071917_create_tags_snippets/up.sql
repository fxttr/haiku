-- Your SQL goes here
CREATE TABLE snippets_tags (
       id SERIAL PRIMARY KEY,
       snippet_id integer,
       tag_id integer,
       constraint fk_snippets_tags
       		  foreign key (snippet_id)
		  REFERENCES snippets (id),
       constraint fk_tags_snippets
       		  foreign key (tag_id)
		  REFERENCES tags (id)
)
