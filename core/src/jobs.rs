use crate::error::CoreResult;
use crate::NeobabuCore;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info, warn};

mod apod;
mod birthday_notification;
mod youtube_update;

pub struct Scheduler {
    core: NeobabuCore,
    scheduler: JobScheduler,
}

impl Scheduler {
    pub async fn new(core: &NeobabuCore) -> CoreResult<Self> {
        let scheduler = JobScheduler::new().await?;
        Ok(Self {
            core: core.clone(),
            scheduler,
        })
    }

    pub async fn start(mut self) -> CoreResult<()> {
        info!("Starting job scheduler...");
        self.schedule().await?;
        self.scheduler.start().await?;
        info!("Job scheduler successfully started");
        Ok(())
    }

    async fn schedule(&mut self) -> CoreResult<()> {
        info!("Scheduling jobs...");
        self.schedule_job("apod", "0 * * * * *", apod::run).await?;
        self.schedule_job(
            "birthday_notification",
            "0 5 9 * * *",
            birthday_notification::run,
        )
        .await?;
        self.schedule_job("youtube_update", "30 * * * * *", youtube_update::run)
            .await?;
        info!("Jobs successfully scheduled");
        Ok(())
    }

    async fn schedule_job<F, Fut>(&mut self, name: &str, cron: &str, job_fn: F) -> CoreResult<()>
    where
        F: Fn(NeobabuCore) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = CoreResult<()>> + Send + 'static,
    {
        let core = self.core.clone();
        let job_name = name.to_string();
        let job_fn = Arc::new(job_fn);
        let is_running = Arc::new(Mutex::new(false));

        let job = Job::new_async(cron, move |_uuid, _lock| {
            let core = core.clone();
            let job_name = job_name.clone();
            let job_fn = Arc::clone(&job_fn);
            let is_running = Arc::clone(&is_running);

            Box::pin(async move {
                let mut running = is_running.lock().await;

                if *running {
                    warn!("Job '{job_name}' is still running, skipping this execution");
                    return;
                }

                *running = true;
                drop(running);

                info!("Running job '{job_name}'");
                let result = job_fn(core).await;

                let mut running = is_running.lock().await;
                *running = false;
                drop(running);

                match result {
                    Ok(_) => info!("Job '{job_name}' finished successfully"),
                    Err(err) => error!("Job '{job_name}' failed: {}", err),
                }
            })
        })?;

        self.scheduler.add(job).await?;
        info!("Registered job '{name}' with schedule '{cron}'");

        Ok(())
    }
}
