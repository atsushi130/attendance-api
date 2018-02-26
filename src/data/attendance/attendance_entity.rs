
use domain::AttendanceModel;
use schema::attendances;

#[derive(Queryable)]
pub struct AttendanceEntity {
    pub id: i32,
    pub user: String,
    pub check_at: String,
    pub attendance_type: i32
}

#[derive(Insertable)]
#[table_name = "attendances"]
pub struct AttendanceInsertableEntity {
    pub user: String,
    pub check_at: String,
    pub attendance_type: i32
}

impl AttendanceInsertableEntity {
    pub fn new(model: &AttendanceModel) -> Self {
        AttendanceInsertableEntity {
            user: model.user.clone(),
            check_at: model.check_at.clone(),
            attendance_type: model.attendance_type
        }
    }
}
