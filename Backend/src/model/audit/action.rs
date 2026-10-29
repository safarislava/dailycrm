use crate::model::project::stage::cost::valid_cost::ValidCost;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuditAction {
    UserCreate,
    InviteCreate,
    ProjectCreate,
    ProjectRename {
        new_title: String,
    },
    ProjectDelete,
    StageCreate,
    StageRename {
        new_title: String,
    },
    StageDelete,
    StageReorder {
        to: i32,
    },
    DeadlineUpdate {
        new_deadline: Option<String>,
    },
    CostUpdate {
        kind: &'static str,
        new_cost: Option<ValidCost>,
    },
    GipConfirm {
        confirmed: bool,
    },
    ActUpload {
        filename: String,
    },
    ActDelete,
    AttachmentUpload {
        filename: String,
    },
    AttachmentDelete {
        filename: String,
    },
    CommentCreate {
        text: String,
    },
    CommentDelete,
}

impl fmt::Display for AuditAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UserCreate => write!(f, "user_create"),
            Self::InviteCreate => write!(f, "invite_create"),
            Self::ProjectCreate => write!(f, "project_create"),
            Self::ProjectRename { new_title } => {
                write!(f, "project_rename(title: '{new_title}')")
            }
            Self::ProjectDelete => write!(f, "project_delete"),
            Self::StageCreate => write!(f, "stage_create"),
            Self::StageRename { new_title } => {
                write!(f, "stage_rename(title: '{new_title}')")
            }
            Self::StageDelete => write!(f, "stage_delete"),
            Self::StageReorder { to } => write!(f, "stage_reorder(to: {to})"),

            Self::DeadlineUpdate { new_deadline } => {
                let d = new_deadline.as_deref().unwrap_or("none");
                write!(f, "deadline_update(deadline: '{d}')")
            }
            Self::CostUpdate { kind, new_cost } => {
                let c = new_cost
                    .as_ref()
                    .map(|v| {
                        v.as_decimal()
                            .map(|d| d.to_string())
                            .unwrap_or_else(|_| "invalid".to_string())
                    })
                    .unwrap_or_else(|| "none".to_string());
                write!(f, "cost_update(kind: '{kind}', cost: {c})")
            }
            Self::GipConfirm { confirmed } => {
                write!(f, "gip_confirm(confirmed: {confirmed})")
            }
            Self::ActUpload { filename } => {
                write!(f, "act_upload(file: '{filename}')")
            }
            Self::ActDelete => write!(f, "act_delete"),
            Self::AttachmentUpload { filename } => {
                write!(f, "attachment_upload(file: '{filename}')")
            }
            Self::AttachmentDelete { filename } => {
                write!(f, "attachment_delete(file: '{filename}')")
            }
            Self::CommentCreate { text } => {
                write!(f, "comment_create(text: '{text}')")
            }
            Self::CommentDelete => write!(f, "comment_delete"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_audit_actions_correctly() {
        assert_eq!(format!("{}", AuditAction::UserCreate), "user_create");
        assert_eq!(
            format!(
                "{}",
                AuditAction::ProjectRename {
                    new_title: "Building A".to_string()
                }
            ),
            "project_rename(title: 'Building A')"
        );
        assert_eq!(
            format!("{}", AuditAction::StageReorder { to: 2 }),
            "stage_reorder(to: 2)"
        );
        assert_eq!(
            format!(
                "{}",
                AuditAction::DeadlineUpdate {
                    new_deadline: Some("2026-12-31".to_string())
                }
            ),
            "deadline_update(deadline: '2026-12-31')"
        );
        assert_eq!(
            format!("{}", AuditAction::DeadlineUpdate { new_deadline: None }),
            "deadline_update(deadline: 'none')"
        );
        assert_eq!(
            format!(
                "{}",
                AuditAction::CostUpdate {
                    kind: "first_advance",
                    new_cost: Some(100.into())
                }
            ),
            "cost_update(kind: 'first_advance', cost: 100)"
        );
        assert_eq!(
            format!(
                "{}",
                AuditAction::CostUpdate {
                    kind: "second_advance",
                    new_cost: Some(250.into())
                }
            ),
            "cost_update(kind: 'second_advance', cost: 250)"
        );
        assert_eq!(
            format!(
                "{}",
                AuditAction::CostUpdate {
                    kind: "final",
                    new_cost: None
                }
            ),
            "cost_update(kind: 'final', cost: none)"
        );
        assert_eq!(
            format!("{}", AuditAction::GipConfirm { confirmed: true }),
            "gip_confirm(confirmed: true)"
        );
        assert_eq!(
            format!(
                "{}",
                AuditAction::ActUpload {
                    filename: "act_01.pdf".to_string()
                }
            ),
            "act_upload(file: 'act_01.pdf')"
        );
    }
}
