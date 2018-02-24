-- Your SQL goes here
CREATE TABLE attendance (
  id INTEGER NOT NULL PRIMARY KEY,
  user VARCHAR NOT NULL,
  check_at DATE NOT NULL,
  attendance_type INTEGER NOT NULL
)

CREATE TABLE attendanceType (
  type INTEGER NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL
)
