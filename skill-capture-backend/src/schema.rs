table! {
    employee (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        title -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    employeeskill (employee_id, skill_id) {
        skill_id -> Int4,
        employee_id -> Int4,
        history -> Nullable<Json>,
    }
}

table! {
    skill (id) {
        id -> Int4,
        name -> Text,
        category -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    employee,
    employeeskill,
    skill,
);
