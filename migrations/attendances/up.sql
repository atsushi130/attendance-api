-- Your SQL goes here
CREATE TABLE attendances (
  id INTEGER NOT NULL PRIMARY KEY,
  user VARCHAR NOT NULL,
  check_at DATE NOT NULL,
  attendance_type INTEGER NOT NULL
)
