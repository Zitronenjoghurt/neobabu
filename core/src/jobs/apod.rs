use crate::error::CoreResult;
use crate::NeobabuCore;
use sea_orm::{IntoActiveModel, Set};
use tracing::info;

pub async fn run(core: NeobabuCore) -> CoreResult<()> {
    if let Some(apod) = core.services.apod.today().await? {
        if !apod.was_announced {
            let mut active = apod.into_active_model();
            active.was_announced = Set(true);
            let apod = core.stores.apod.update(active).await?;
            info!("NEW APOD ANNOUNCED: {:?}", apod.title);
            core.event_bus
                .send(crate::events::CoreEvent::new_apod(apod));
        } else {
            info!("APOD already announced today: {:?}", apod.title);
        }
    } else {
        info!("No APOD found for today yet");
    }
    Ok(())
}
