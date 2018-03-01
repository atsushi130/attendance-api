
use chrono::{ DateTime, Local };

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

    pub fn get_check_at_date(&self) -> DateTime<Local> {
        DateTime::from_str(&self.check_at, "%Y/%m/%d %H:%M:%S").unwrap()
    }
}
