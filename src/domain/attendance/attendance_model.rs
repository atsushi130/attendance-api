
pub struct AttendanceModel {
    pub user: String,
    pub check_at: String,
    pub attendance_type: i32
}

impl AttendanceModel {
    pub fn new(user: &str, check_at: &str, attendance_type: i32) -> Self {
        AttendanceModel {
            user: user.to_string(),
            check_at: check_at.to_string(),
            attendance_type
        }
    }
}
