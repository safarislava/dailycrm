use crate::model::user::role::Role;

#[derive(sqlx::FromRow)]
pub struct QueuedNotification {
    #[sqlx(rename = "type")]
    notification_type: String,
    project_title: String,
    stage_title: String,
}

impl QueuedNotification {
    pub fn role(&self) -> Option<Role> {
        match self.notification_type.as_str() {
            "work_complete" => Some(Role::Lawyer),
            "act_uploaded" => Some(Role::Accountant),
            _ => None,
        }
    }

    pub fn subject(&self) -> Option<&str> {
        match self.notification_type.as_str() {
            "work_complete" => Some("DailyCRM: Работа выполнена"),
            "act_uploaded" => Some("DailyCRM: Загружен акт"),
            _ => None,
        }
    }

    pub fn body(&self) -> Option<String> {
        match self.notification_type.as_str() {
            "work_complete" => Some(format!(
                "ГИП отметил выполнение этапа «{}» в проекте «{}».\n\nПожалуйста, загрузите акт.",
                self.stage_title, self.project_title
            )),
            "act_uploaded" => Some(format!(
                "Юрист загрузил акт для этапа «{}» в проекте «{}».\n\nПожалуйста, подтвердите оплату.",
                self.stage_title, self.project_title
            )),
            _ => None,
        }
    }
}
