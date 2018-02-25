
use database::DatabaseConnection;

pub struct AttendanceService {
    connection: DatabaseConnection
}

impl AttendanceService {
    pub fn new(connection: DatabaseConnection) -> AttendanceService {
        AttendanceService {
            connection
        }
    }
}