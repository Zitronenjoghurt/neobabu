use crate::database::entity::{economy, economy_pending, user};
use crate::database::Database;
use crate::error::CoreResult;
use crate::types::currency::Currency;
use sea_orm::prelude::Expr;
use sea_orm::sea_query::{Alias, Func, Query, SimpleExpr, SubQueryStatement};
use sea_orm::ExprTrait;
use sea_orm::{ActiveModelTrait, EntityTrait, FromQueryResult, Set, TransactionTrait};
use sea_orm::{ConnectionTrait, IntoActiveModel};
use std::sync::Arc;

pub struct EconomyStore {
    db: Arc<Database>,
}

impl EconomyStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    async fn find_or_create(
        &self,
        txn: &impl ConnectionTrait,
        user: &user::Model,
        currency: Currency,
    ) -> CoreResult<economy::Model> {
        if let Some(economy) = economy::Entity::find_by_id((user.id.to_string(), currency.into()))
            .one(txn)
            .await?
        {
            return Ok(economy);
        };

        let new = economy::ActiveModel {
            user_id: Set(user.id.to_string()),
            currency: Set(currency.into()),
            amount: Set(0),
        };
        Ok(new.insert(txn).await?)
    }

    async fn balance_in_txn(
        &self,
        txn: &impl ConnectionTrait,
        user_id: &str,
        currency_val: i16,
    ) -> CoreResult<i64> {
        let locked_subquery = Query::select()
            .expr(Func::coalesce([
                Expr::col((economy_pending::Entity, economy_pending::Column::Amount)).sum(),
                Expr::value(0),
            ]))
            .from(economy_pending::Entity)
            .and_where(Expr::col(economy_pending::Column::UserId).eq(user_id))
            .and_where(Expr::col(economy_pending::Column::Currency).eq(currency_val))
            .and_where(Expr::col(economy_pending::Column::ExpiresAt).gt(Expr::current_timestamp()))
            .to_owned();

        let query = Query::select()
            .expr_as(
                Expr::col((economy::Entity, economy::Column::Amount))
                    .sub(SimpleExpr::SubQuery(
                        None,
                        Box::new(SubQueryStatement::from(locked_subquery)),
                    ))
                    .cast_as(Alias::new("BIGINT")),
                Alias::new("available"),
            )
            .from(economy::Entity)
            .and_where(Expr::col(economy::Column::UserId).eq(user_id))
            .and_where(Expr::col(economy::Column::Currency).eq(currency_val))
            .to_owned();

        let result = AvailableBalance::find_by_statement(txn.get_database_backend().build(&query))
            .one(txn)
            .await?
            .unwrap_or(AvailableBalance { available: 0 });

        Ok(result.available)
    }

    pub async fn balance(&self, user: &user::Model, currency: Currency) -> CoreResult<i64> {
        self.balance_in_txn(self.db.conn(), &user.id, currency.into())
            .await
    }

    pub async fn reserve(
        &self,
        reference_id: impl AsRef<str>,
        timeout_duration: std::time::Duration,
        user: &user::Model,
        currency: Currency,
        amount: i64,
    ) -> CoreResult<bool> {
        if amount <= 0 {
            return Ok(false);
        };

        let txn = self.db.conn().begin().await?;
        let available = self.balance_in_txn(&txn, &user.id, currency.into()).await?;

        if available < amount {
            txn.rollback().await?;
            return Ok(false);
        };

        let expires_at = chrono::Utc::now() + timeout_duration;
        let pending = economy_pending::ActiveModel {
            reference_id: Set(reference_id.as_ref().to_string()),
            user_id: Set(user.id.to_string()),
            currency: Set(currency.into()),
            amount: Set(amount),
            expires_at: Set(expires_at.naive_utc()),
            ..Default::default()
        };

        pending.insert(&txn).await?;
        txn.commit().await?;

        Ok(true)
    }

    pub async fn commit(
        &self,
        reference_id: impl AsRef<str>,
        user: &user::Model,
        currency: Currency,
    ) -> CoreResult<bool> {
        let txn = self.db.conn().begin().await?;

        let Some(pending) = economy_pending::Entity::find_by_id((
            reference_id.as_ref().to_string(),
            user.id.to_string(),
            currency.into(),
        ))
        .one(&txn)
        .await?
        else {
            txn.rollback().await?;
            return Ok(false);
        };

        if pending.expires_at < chrono::Utc::now().naive_utc() {
            txn.rollback().await?;
            return Ok(false);
        };

        economy_pending::Entity::delete_by_id((
            reference_id.as_ref().to_string(),
            user.id.to_string(),
            currency.into(),
        ))
        .exec(&txn)
        .await?;

        let economy = self.find_or_create(&txn, user, currency).await?;
        let current_amount = economy.amount;

        if current_amount < pending.amount {
            txn.rollback().await?;
            return Ok(false);
        }

        let mut active = economy.into_active_model();
        active.amount = Set(current_amount.saturating_sub(pending.amount));
        active.update(&txn).await?;

        txn.commit().await?;
        Ok(true)
    }

    pub async fn cancel(
        &self,
        reference_id: impl AsRef<str>,
        user: &user::Model,
        currency: Currency,
    ) -> CoreResult<()> {
        economy_pending::Entity::delete_by_id((
            reference_id.as_ref().to_string(),
            user.id.to_string(),
            currency.into(),
        ))
        .exec(self.db.conn())
        .await?;

        Ok(())
    }

    pub async fn add(&self, user: &user::Model, currency: Currency, amount: i64) -> CoreResult<()> {
        let txn = self.db.conn().begin().await?;
        let economy = self.find_or_create(&txn, user, currency).await?;
        let current_amount = economy.amount;

        let mut active = economy.into_active_model();
        active.amount = Set(current_amount.saturating_add(amount));
        active.update(&txn).await?;
        txn.commit().await?;

        Ok(())
    }

    pub async fn subtract(
        &self,
        user: &user::Model,
        currency: Currency,
        amount: i64,
    ) -> CoreResult<bool> {
        let txn = self.db.conn().begin().await?;
        let economy = self.find_or_create(&txn, user, currency).await?;
        let current_amount = economy.amount;

        if current_amount < amount {
            txn.rollback().await?;
            return Ok(false);
        }

        let mut active = economy.into_active_model();
        active.amount = Set(current_amount.saturating_sub(amount));
        active.update(&txn).await?;
        txn.commit().await?;

        Ok(true)
    }
}

#[derive(FromQueryResult)]
struct AvailableBalance {
    available: i64,
}
