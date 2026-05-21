use std::sync::Arc;
use crate::stage::Stage;

pub struct Stages {
    stages: Arc<[Stage]>,
}