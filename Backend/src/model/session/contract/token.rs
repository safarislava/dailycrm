use crate::model::credential::contract::contentable::Contentable;

pub trait Token: Contentable<Output = String> {}

impl Token for String {}
