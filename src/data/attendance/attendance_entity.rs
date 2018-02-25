
use schema::attendances;

#[derive(Queryable)]
pub struct AttendanceEntity {
    pub id: i32,
    pub user: String,
    pub check_at: String,
    pub attendance_type: i32
}

#[derive(Insertable)]
#[table_name = "attendances"]
pub struct AttendanceInsertableEntity {
    pub user: String,
    pub check_at: String,
    pub attendance_type: i32
}

impl AttendanceInsertableEntity {
    pub fn new(user: &str, check_at: &str, attendance_type: i32) -> Self {
        AttendanceInsertableEntity {
            user: user.to_string(),
            check_at: check_at.to_string(),
            attendance_type
        }
    }
}
