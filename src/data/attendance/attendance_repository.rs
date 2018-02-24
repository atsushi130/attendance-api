
use super::AttendanceEntity;
use diesel::SqliteConnection;
use schema::attendances::dsl::*;
use diesel::prelude::*;

pub struct AttendanceRepository {
    connection: SqliteConnection
}

impl AttendanceRepository {
    pub fn from(connection: SqliteConnection) -> Self {
        AttendanceRepository {
            connection
        }
    }
}

impl AttendanceRepository {

    pub fn getAttendances(&self) -> Vec<AttendanceEntity> {
        attendances.load::<AttendanceEntity>(&self.connection).expect("Error loading attendances")
    }

    pub fn register(&self, entity: &AttendanceEntity) {
    }
}
