
use super::AttendanceEntity;
use database::DatabaseConnection;
use schema::attendances::dsl::*;
use diesel::prelude::*;
use diesel::associations::HasTable;

pub struct AttendanceRepository {
    connection: DatabaseConnection
}

impl AttendanceRepository {
    pub fn new(connection: DatabaseConnection) -> Self {
        AttendanceRepository {
            connection
        }
    }
}

impl AttendanceRepository {

    pub fn get_attendances(&self) -> Vec<AttendanceEntity> {
        attendances.load::<AttendanceEntity>(&*self.connection).expect("Error loading attendances")
    }

    pub fn get_attendances_by_user(&self, user_name: &str) -> Vec<AttendanceEntity> {
        attendances.filter(user.eq(user_name)).load::<AttendanceEntity>(&*self.connection).expect("Error")
    }

    pub fn register(&self, entity: &AttendanceEntity) {
    }
}
