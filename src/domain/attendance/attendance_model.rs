
pub struct AttendanceModel {
    user: String,
    check_at: String,
    attendance_type: i32
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
