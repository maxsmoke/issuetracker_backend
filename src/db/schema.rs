table! {
    issue (id) {
        id -> Integer,
        title -> Text,
        project_id -> Integer,
        complete -> Integer,
        content -> Text,
    }
}

table! {
    project (id) {
        id -> Integer,
        title -> Text,
        complete -> Integer,
        issue_count -> Integer,
    }
}

joinable!(issue -> project (project_id));

allow_tables_to_appear_in_same_query!(
    issue,
    project,
);
