use std::sync::Arc;
use crate::models::stage::Stage;

pub struct Stages {
    stages: Arc<[Stage]>,
}