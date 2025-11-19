use crate::database::entity::{rps_games, user};
use crate::database::Database;
use crate::error::CoreResult;
use sea_orm::sea_query::*;
use sea_orm::*;
use std::sync::Arc;

pub struct RPSGamesStore {
    db: Arc<Database>,
}

impl RPSGamesStore {
    pub fn initialize(db: &Arc<Database>) -> Arc<Self> {
        Arc::new(Self { db: db.clone() })
    }

    fn id_pair(&self, user_id_1: impl AsRef<str>, user_id_2: impl AsRef<str>) -> (String, String) {
        let user_id_1 = user_id_1.as_ref().to_owned();
        let user_id_2 = user_id_2.as_ref().to_owned();

        if user_id_1 < user_id_2 {
            (user_id_1, user_id_2)
        } else {
            (user_id_2, user_id_1)
        }
    }

    pub async fn find_by_id(
        &self,
        user_1_id: impl AsRef<str>,
        user_2_id: impl AsRef<str>,
    ) -> CoreResult<Option<rps_games::Model>> {
        Ok(
            rps_games::Entity::find_by_id(self.id_pair(user_1_id, user_2_id))
                .one(self.db.conn())
                .await?,
        )
    }

    pub async fn fetch_or_create(
        &self,
        user_1: &user::Model,
        user_2: &user::Model,
    ) -> CoreResult<rps_games::Model> {
        let id = self.id_pair(&user_1.id, &user_2.id);
        if let Some(existing) = self.find_by_id(&id.0, &id.1).await? {
            return Ok(existing);
        };

        let new = rps_games::ActiveModel {
            user_id1: Set(id.0),
            user_id2: Set(id.1),
            ..Default::default()
        };

        Ok(new.insert(self.db.conn()).await?)
    }

    pub async fn update(&self, mut model: rps_games::ActiveModel) -> CoreResult<rps_games::Model> {
        model.updated_at = Set(chrono::Utc::now().naive_utc());
        Ok(model.update(self.db.conn()).await?)
    }

    pub async fn get_user_stats(&self, user_id: impl AsRef<str>) -> CoreResult<RPSUserStats> {
        let user_id = user_id.as_ref();

        let mut subquery1 = Query::select()
            .expr_as(
                Expr::col((rps_games::Entity, rps_games::Column::Wins1)),
                Alias::new("wins"),
            )
            .expr_as(
                Expr::col((rps_games::Entity, rps_games::Column::Wins2)),
                Alias::new("losses"),
            )
            .expr_as(
                Expr::col((rps_games::Entity, rps_games::Column::Draws)),
                Alias::new("draws"),
            )
            .from(rps_games::Entity)
            .and_where(Expr::col(rps_games::Column::UserId1).eq(user_id))
            .to_owned();

        let subquery2 = Query::select()
            .expr_as(
                Expr::col((rps_games::Entity, rps_games::Column::Wins2)),
                Alias::new("wins"),
            )
            .expr_as(
                Expr::col((rps_games::Entity, rps_games::Column::Wins1)),
                Alias::new("losses"),
            )
            .expr_as(
                Expr::col((rps_games::Entity, rps_games::Column::Draws)),
                Alias::new("draws"),
            )
            .from(rps_games::Entity)
            .and_where(Expr::col(rps_games::Column::UserId2).eq(user_id))
            .to_owned();

        let union_query = subquery1.union(UnionType::All, subquery2).to_owned();

        let final_query = Query::select()
            .expr_as(Expr::cust("COALESCE(SUM(wins), 0)"), Alias::new("wins"))
            .expr_as(Expr::cust("COALESCE(SUM(losses), 0)"), Alias::new("losses"))
            .expr_as(Expr::cust("COALESCE(SUM(draws), 0)"), Alias::new("draws"))
            .from_subquery(union_query, Alias::new("combined_stats"))
            .to_owned();

        let result = RPSUserStats::find_by_statement(
            self.db.conn().get_database_backend().build(&final_query),
        )
        .one(self.db.conn())
        .await?
        .unwrap_or(RPSUserStats {
            wins: 0,
            losses: 0,
            draws: 0,
        });

        Ok(result)
    }
}

#[derive(Debug, FromQueryResult)]
pub struct RPSUserStats {
    pub wins: i64,
    pub losses: i64,
    pub draws: i64,
}
