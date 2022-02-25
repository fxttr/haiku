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
