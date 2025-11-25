use crate::cryptor::Cryptor;
use crate::database::entity::user;
use crate::error::{CoreError, CoreResult};
use crate::integrations::apis::discord::DiscordClient;
use crate::services::birthday::BirthdayService;
use crate::stores::Stores;
use crate::types::user_guild_info::UserGuildInfo;
use crate::types::user_settings::birthday::UserBirthdaySettings;
use crate::types::user_settings::UserSettings;
use serenity::model::Permissions;
use std::collections::HashMap;
use std::sync::Arc;

pub struct UserService {
    stores: Arc<Stores>,
}

impl UserService {
    pub fn initialize(stores: &Arc<Stores>) -> Arc<Self> {
        Arc::new(Self {
            stores: stores.clone(),
        })
    }

    pub async fn guild_infos(
        &self,
        user: &user::Model,
        cryptor: &Arc<Cryptor>,
    ) -> CoreResult<Vec<UserGuildInfo>> {
        let Some(encrypted_token) = &user.encrypted_oauth_token else {
            return Err(CoreError::Unauthorized);
        };
        let decrypted_token = cryptor.decrypt(encrypted_token)?;

        let client = DiscordClient::with_user_token(decrypted_token);
        let guilds = client.guilds().collect().await?;
        let guild_ids = guilds.iter().map(|guild| guild.id.to_string());

        let bot_guilds = self
            .stores
            .guild
            .with_ids(guild_ids.clone())
            .await?
            .into_iter()
            .map(|guild| (guild.id.clone(), guild))
            .collect::<HashMap<_, _>>();
        let user_guilds = self
            .stores
            .user_guild
            .with_guild_ids(guild_ids)
            .await?
            .into_iter()
            .map(|user_guild| (user_guild.guild_id.clone(), user_guild))
            .collect::<HashMap<_, _>>();

        let mut infos = Vec::new();
        for guild in guilds {
            let guild_id = guild.id.to_string();
            let has_bot = bot_guilds.contains_key(&guild_id);
            let is_active = user_guilds.contains_key(&guild_id);
            let can_manage = guild.permissions.contains(Permissions::MANAGE_GUILD);

            if !has_bot && !is_active && !can_manage {
                continue;
            }

            let info = UserGuildInfo {
                id: guild_id,
                name: guild.name,
                icon_hash: guild.icon.map(|icon| icon.to_string()),
                has_bot,
                is_active,
                can_manage,
            };

            infos.push(info);
        }

        Ok(infos)
    }

    pub async fn get_settings(
        &self,
        birthday_service: &Arc<BirthdayService>,
        user: &user::Model,
    ) -> CoreResult<UserSettings> {
        let user_birthday = self.stores.user_birthday.find_by_user_id(&user.id).await?;
        let birthday = user_birthday.map(|ub| UserBirthdaySettings {
            day: ub.day,
            month: ub.month,
            year: ub.year,
            updatable: birthday_service.can_update(&ub),
        });
        Ok(UserSettings { birthday })
    }
}
