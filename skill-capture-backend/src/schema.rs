diesel::table! {
    employee (id) {
        id -> Int4,
        firstname -> Text,
        lastname -> Text,
        title -> Text,
        createdat -> Timestamp,
        updatedat -> Timestamp,
    }
}
