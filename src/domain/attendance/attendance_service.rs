
use database::DatabaseConnection;
use data::AttendanceRepository;
use super::AttendanceModel;

pub struct AttendanceService;

type AttendanceModels = Vec<AttendanceModel>;

impl AttendanceService {

    pub fn get_attendances(&self, connection: DatabaseConnection) -> AttendanceModels {
        AttendanceRepository::new(connection).get_attendances().into_iter().map(|entity|
            AttendanceModel::new(&entity.user, &entity.check_at, entity.attendance_type)
        ).collect()
    }

    pub fn get_attendances_by_user(&self, user: &str, connection: DatabaseConnection) -> AttendanceModels {
        AttendanceRepository::new(connection).get_attendances_by_user(&user).into_iter().map(|entity|
            AttendanceModel::new(&entity.user, &entity.check_at, entity.attendance_type)
        ).collect()
    }

    pub fn get_attendances_by_month(&self, user: &str, month: u32, connection: DatabaseConnection) -> AttendanceModels {
        AttendanceRepository::new(connection).get_attendances_by_month(&user, month).into_iter().map(|entity|
            AttendanceModel::new(&entity.user, &entity.check_at, entity.attendance_type)
        ).collect()
    }

    pub fn get_working_time(&self, user: &str, month: u32, connection: DatabaseConnection) -> i32 {
        let _ = self.get_attendances_by_month(user, month, connection);
        1
    }

    pub fn get_working_overtime(&self, user: &str, month: u32, connection: DatabaseConnection) -> i32 {
        let working_time = self.get_working_time(user, month, connection);
        // TODO:
        8 * 20 - working_time
    }
}
