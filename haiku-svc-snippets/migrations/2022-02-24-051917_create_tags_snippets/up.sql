-- Your SQL goes here
CREATE TABLE snippets_tags (
       id SERIAL PRIMARY KEY,
       snippet_id integer,
       tag_id integer
)
