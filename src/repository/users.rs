use macroses::NewTypeDeref;
use sqlx::{Pool, Postgres};
use std::{ops::Deref, sync::Arc};
use uuid::Uuid;

use crate::{models::users::User, schemas::users::RegisterUser};

#[derive(NewTypeDeref)]
pub struct Limit(pub u32);
#[derive(NewTypeDeref)]
pub struct Offset(pub u32);

pub trait UserRepository {
    async fn get_by_id(&self, id: &Uuid) -> Option<User>;
    async fn get(&self, offset: &Offset, limit: &Limit) -> Option<User>;
    async fn create(&self, user: RegisterUser);
    async fn update(&self, user: User);
}

#[derive(Clone)]
pub struct UserRepo {
    db_pool: Arc<Pool<Postgres>>,
}

impl UserRepo {
    pub fn new(db_pool: Arc<Pool<Postgres>>) -> Self {
        Self { db_pool }
    }
}


