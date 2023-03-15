// @generated automatically by Diesel CLI.

diesel::table! {
    exercises (id) {
        id -> Int4,
        points -> Int4,
    }
}

diesel::table! {
    scoreboard (id) {
        id -> Int4,
        team -> Text,
        exercise_id -> Int4,
    }
}

diesel::joinable!(scoreboard -> exercises (exercise_id));

diesel::allow_tables_to_appear_in_same_query!(
    exercises,
    scoreboard,
);
