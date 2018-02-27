
use domain::AttendanceModel;

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

    pub fn to_model(&self) -> AttendanceModel {
        AttendanceModel {
            user: self.user.clone(),
            check_at: self.check_at.clone(),
            attendance_type: self.attendance_type
        }
    }
}