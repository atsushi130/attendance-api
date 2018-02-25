
//#[derive(Serialize, Deserialize)]
#[derive(Serialize, Deserialize)]
pub struct AttendanceResource {
    pub user: String,
    pub check_at: String,
    pub attendance_type: i32
}

impl AttendanceResource {
    pub fn new(user: &str, check_at: &str, attendance_type: i32) -> Self {
        AttendanceResource {
            user: user.to_string(),
            check_at: check_at.to_string(),
            attendance_type
        }
    }
}