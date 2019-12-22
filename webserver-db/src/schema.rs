table! {
    pages (id) {
        id -> Int4,
        parent_id -> Int4,
        name -> Varchar,
        body -> Text,
        is_folder -> Bool,
        published -> Bool,
    }
}
