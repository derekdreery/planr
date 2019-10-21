table! {
    project (id) {
        id -> Int4,
        parent_project_id -> Nullable<Int4>,
        name -> Text,
        due_date -> Nullable<Timestamptz>,
    }
}

table! {
    task (id) {
        id -> Int4,
        project_id -> Int4,
        description -> Text,
        due_date -> Nullable<Timestamptz>,
        completed -> Bool,
    }
}

table! {
    task_sequence (before_task_id, after_task_id) {
        before_task_id -> Int4,
        after_task_id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    project,
    task,
    task_sequence,
);
