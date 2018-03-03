
use database::DatabaseConnection;
use data::{ AttendanceRepository, AttendanceInsertableEntity };
use reduce::Reduce;
use super::AttendanceModel;

pub struct AttendanceService;

type AttendanceModels = Vec<AttendanceModel>;

impl AttendanceService {

    pub fn register(&self, attendance: &AttendanceModel, connection: DatabaseConnection) {
        let entity = AttendanceInsertableEntity::new(attendance);
        AttendanceRepository::new(connection).register(&entity);
    }

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

    pub fn get_working_time(&self, user: &str, month: u32, connection: DatabaseConnection) -> f32 {

        let mut working_time = 0.0;
        self.get_attendances_by_month(user, month, connection).into_iter().reduce(|lhs, rhs| {
            println!("{}", lhs.get_working_time(&rhs));
            working_time += lhs.get_working_time(&rhs);
            rhs
        });

        working_time
    }

    pub fn get_working_overtime(&self, user: &str, month: u32, connection: DatabaseConnection) -> f32 {

        let working_time     = self.get_working_time(user, month, connection);
        let working_overtime = working_time - ((8 * 20) as f32);

        if working_overtime >= 0.0 {
            working_overtime
        } else {
            0.0
        }
    }
}
