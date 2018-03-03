
use chrono::{ DateTime, Local, TimeZone, Duration, Timelike, Datelike };
use std::str::FromStr;
use std::fmt::Debug;

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
        Local.datetime_from_str(&self.check_at, "%Y/%m/%d %H:%M:%S").unwrap()
    }

    pub fn get_working_time(&self, attendance: &AttendanceModel) -> f32 {

        if !self.validate(attendance) {
            return 0.0
        }

        let from = self.get_check_at_date();
        let to   = attendance.get_check_at_date();


        let duration = self.get_duration();
        let date_time = to - duration;

        println!("{}", date_time.to_string());

        (date_time.hour() as f32) + (date_time.minute() as f32 / 60.0)
    }

    fn validate(&self, attendance: &AttendanceModel) -> bool {

        if !self.is_valid_working(attendance) {
            return false
        }

        if !self.is_equal_day(attendance) {
            return false
        }

        true
    }

    fn is_equal_day(&self, attendance: &AttendanceModel) -> bool {

        let from = self.get_check_at_date();
        let to   = attendance.get_check_at_date();

        let year  = from.year()  == to.year();
        let month = from.month() == to.month();
        let day   = from.day()   == to.day();

        year && month && day
    }

    fn is_valid_working(&self, attendance: &AttendanceModel) -> bool {
        (self.attendance_type == 0) && (attendance.attendance_type == 1)
    }

    fn get_duration(&self) -> Duration {
        let check_at = self.get_check_at_date();
        println!("duration check at: {}", check_at.to_string());
        (Duration::hours(check_at.hour() as i64) + Duration::minutes(check_at.minute() as i64) + Duration::seconds(check_at.second() as i64))
    }
}
