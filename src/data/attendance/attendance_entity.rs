
#[derive(Queryable)]
pub struct AttendanceEntity {
    pub id: i32,
    pub user: String,
    pub check_at: String,
    pub attendance_type: i32
}