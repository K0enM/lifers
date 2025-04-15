-- Add migration script here
CREATE TYPE workout_type AS ENUM ('walking', 'cycling', 'running');

CREATE TABLE workout_schedule_item (
    id uuid primary key not null,
    user_id uuid not null,
    kind workout_type not null,
    weekday TEXT unique not null
);
