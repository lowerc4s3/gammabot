use std::collections::HashSet;

use teloxide::dispatching::UpdateHandler;
use teloxide::dptree;
use teloxide::prelude::*;
use teloxide::types::{Update, User};

pub mod admin;
pub mod user;

#[derive(Debug, Clone, Copy)]
pub enum UserType {
    Paid,
    Free,
    Admin,
}

pub fn schema() -> UpdateHandler<anyhow::Error> {
    dptree::entry()
        .filter_map(|upd: Update| upd.from().cloned())
        .branch(dptree::filter(admin_filter).chain(admin::schema()))
        .branch(user::schema())
}

fn admin_filter(admin_ids: HashSet<UserId>, user: User) -> bool {
    admin_ids.contains(&user.id)
}
