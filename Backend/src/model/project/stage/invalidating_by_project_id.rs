use crate::model::cache::contract::cache::Cache;
use crate::model::cache::invalidating_task::InvalidatingTask;
use crate::model::contract::box_error::BoxError;
use crate::model::contract::task::Task;
use crate::model::project::id::ProjectId;
use crate::model::project::stage::cache_key::StageCacheKey;
use crate::model::project::stage::collecting_media::StageSummaryItem;
use async_trait::async_trait;

pub struct InvalidatingByProjectId<T, C> {
    task: InvalidatingTask<T, C, StageCacheKey, Vec<StageSummaryItem>>,
}

impl<T, C> InvalidatingByProjectId<T, C> {
    pub fn new(origin: T, cache: C, project_id: ProjectId) -> Self {
        Self {
            task: InvalidatingTask::single(origin, cache, StageCacheKey::ByProjectId(project_id)),
        }
    }
}

#[async_trait]
impl<T, C> Task for InvalidatingByProjectId<T, C>
where
    T: Task + Send + Sync,
    C: Cache<StageCacheKey, Vec<StageSummaryItem>> + Send + Sync,
{
    type Output = T::Output;

    async fn perform(&self) -> Result<Self::Output, BoxError> {
        self.task.perform().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::cache::memory_cache::MemoryCache;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use uuid::Uuid;

    struct FlagTask(Arc<AtomicBool>);

    #[async_trait]
    impl Task for FlagTask {
        type Output = ();
        async fn perform(&self) -> Result<(), BoxError> {
            self.0.store(true, Ordering::SeqCst);
            Ok(())
        }
    }

    #[actix_web::test]
    async fn evicts_stage_summary_by_project_id_from_cache() {
        let cache = MemoryCache::new();
        let project_id = ProjectId::new(Uuid::new_v4());

        cache
            .save(StageCacheKey::ByProjectId(project_id), vec![])
            .await
            .unwrap();

        let flag = Arc::new(AtomicBool::new(false));
        let decorator =
            InvalidatingByProjectId::new(FlagTask(flag.clone()), cache.clone(), project_id);

        decorator.perform().await.unwrap();

        assert!(flag.load(Ordering::SeqCst));
        assert!(
            cache
                .value(&StageCacheKey::ByProjectId(project_id))
                .await
                .unwrap()
                .is_none()
        );
    }
}
