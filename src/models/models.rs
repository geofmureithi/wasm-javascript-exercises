use diesel::prelude::*;

#[derive(Debug, Clone, Queryable)]
pub struct Exercise {
    pub id: i32,
    pub points: i32,
}

#[derive(Debug, Clone, Queryable)]
pub struct ScoreBoardEntry {
    pub id: i32,
    pub team: String,
    pub exercise_id: i32,
}

#[derive(Debug, Clone, Queryable)]
pub struct TeamWithPoints {
    pub team: String,
    pub points: i32,
}
