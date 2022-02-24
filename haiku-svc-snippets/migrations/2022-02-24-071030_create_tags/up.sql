-- Your SQL goes here
CREATE TABLE tags (
       id SERIAL PRIMARY KEY,
       name VARCHAR(20) NOT NULL,
       constraint fk_tags_snippets_cross
       		  foreign key (id)
		  REFERENCES snippets_tags (snippet_id)
)
