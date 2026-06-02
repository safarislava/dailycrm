use crate::common::BoxError;
use crate::model::project::attachment::Attachment;
use crate::model::task::contract::task::Task;
use crate::model::task::project::attachment_receipt::AttachmentReceipt;
use crate::model::task::project::attachment_removal::AttachmentRemoval;
use crate::model::task::project::system_comment_creation::SystemCommentCreation;
use crate::model::user::user::User;
use crate::storage::Storage;
use sqlx::PgPool;
use std::sync::Arc;

pub struct LoggedAttachmentRemoval {
    pool: Arc<PgPool>,
    storage: Arc<Storage>,
    attachment: Attachment,
    user: User,
}

impl LoggedAttachmentRemoval {
    pub fn new(
        pool: Arc<PgPool>,
        storage: Arc<Storage>,
        attachment: Attachment,
        user: User,
    ) -> Self {
        Self {
            pool,
            storage,
            attachment,
            user,
        }
    }
}

#[async_trait::async_trait]
impl Task for LoggedAttachmentRemoval {
    type Output = ();

    async fn done(&self) -> Result<Self::Output, BoxError> {
        let info = AttachmentReceipt::new(self.pool.clone(), self.attachment.clone())
            .done()
            .await?;
        AttachmentRemoval::new(
            self.pool.clone(),
            self.storage.clone(),
            self.attachment.clone(),
        )
        .done()
        .await?;
        if let Some((filename, stage, is_act)) = info {
            let text = if is_act {
                format!("Удалён акт: {}", filename)
            } else {
                format!("Удалён файл: {}", filename)
            };
            let _ = SystemCommentCreation::new(self.pool.clone(), stage, self.user.clone(), text)
                .done()
                .await;
        }
        Ok(())
    }
}
