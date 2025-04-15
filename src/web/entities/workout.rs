use serde::{Deserialize, Serialize};
use sqlx::prelude::*;
use time::Weekday;
use uuid::Uuid;

#[derive(sqlx::Type)]
#[sqlx(type_name = "workout_type", rename_all = "lowercase")]
#[derive(Debug, Serialize, Deserialize, Clone)]
enum WorkoutType {
    Walking,
    Cycling,
    Running,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
struct WorkoutScheduleItem {
    id: Uuid,
    user_id: Uuid,
    kind: WorkoutType,
    day: Weekday,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateWorkoutScheduleItem {
    user_id: Uuid,
    kind: WorkoutType,
    day: Weekday,
}
