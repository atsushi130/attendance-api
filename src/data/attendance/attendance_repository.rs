
use super::AttendanceEntity;
use database::DatabaseConnection;
use extension::ToTwoDigits;
use chrono::{ Local, Datelike, DateTime, Duration, Weekday, TimeZone };
use chrono::naive::{ NaiveDate, NaiveDateTime };
use schema::attendances::dsl::*;
use diesel::prelude::*;

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

    pub fn get_attendances_by_month(&self, user_name: &str, month: u32) -> Vec<AttendanceEntity> {

        let year = Local::now().year();
        let to_date = Local.ymd(year, month + 1, 1).and_hms(23, 59, 59) - Duration::days(1);

        attendances
            .filter(user.eq(user_name))
            .filter(check_at.ge(format!("{}/{}/01 00:00:00", year, month.to_two_digits())))
            .filter(check_at.lt(format!("{}/{}/{} 23:59:59", year, to_date.month().to_two_digits(), to_date.day())))
            .load::<AttendanceEntity>(&*self.connection).expect("Error")
    }

    pub fn register(&self, entity: &AttendanceEntity) {
    }
}
