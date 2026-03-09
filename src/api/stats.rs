use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::sync::Arc;
use tokio::sync::oneshot;

#[napi]
pub mod stats {
    use napi::bindgen_prelude::AsyncTask;

    #[napi]
    pub fn get_int(name: String) -> Option<i32> {
        let client = crate::client::get_client();
        client.user_stats().get_stat_i32(&name).ok()
    }

    #[napi]
    pub fn set_int(name: String, value: i32) -> bool {
        let client = crate::client::get_client();
        client.user_stats().set_stat_i32(&name, value).is_ok()
    }

    #[napi]
    pub fn store() -> bool {
        let client = crate::client::get_client();
        client.user_stats().store_stats().is_ok()
    }

    #[napi]
    pub fn reset_all(achievements_too: bool) -> bool {
        let client = crate::client::get_client();
        client
            .user_stats()
            .reset_all_stats(achievements_too)
            .is_ok()
    }

    /// Asynchronously requests global stats data for aggregated stats.
    /// Returns the game ID if successful.
    #[napi(ts_return_type = "Promise<bigint>")]
    pub fn request_global_stats(history_days: i32) -> AsyncTask<super::RequestGlobalStatsTask> {
        AsyncTask::new(super::RequestGlobalStatsTask { history_days })
    }

    /// Gets the lifetime total for an aggregated stat as an i64.
    /// Returns null if the stat doesn't exist or hasn't been received yet.
    #[napi]
    pub fn get_global_int(name: String) -> Option<i64> {
        let client = crate::client::get_client();
        client.user_stats().get_global_stat_i64(&name).ok()
    }

    /// Gets the lifetime total for an aggregated stat as an f64.
    /// Returns null if the stat doesn't exist or hasn't been received yet.
    #[napi]
    pub fn get_global_float(name: String) -> Option<f64> {
        let client = crate::client::get_client();
        client.user_stats().get_global_stat_f64(&name).ok()
    }

    /// Gets history for an aggregated stat as i64 values.
    /// Returns daily values starting with today (index 0).
    #[napi]
    pub fn get_global_int_history(name: String, max_days: u32) -> Option<Vec<i64>> {
        let client = crate::client::get_client();
        client
            .user_stats()
            .get_global_stat_history_i64(&name, max_days as usize)
            .ok()
    }

    /// Gets history for an aggregated stat as f64 values.
    /// Returns daily values starting with today (index 0).
    #[napi]
    pub fn get_global_float_history(name: String, max_days: u32) -> Option<Vec<f64>> {
        let client = crate::client::get_client();
        client
            .user_stats()
            .get_global_stat_history_f64(&name, max_days as usize)
            .ok()
    }
}

pub struct RequestGlobalStatsTask {
    history_days: i32,
}

#[napi]
impl Task for RequestGlobalStatsTask {
    type Output = u64;
    type JsValue = BigInt;

    fn compute(&mut self) -> Result<Self::Output> {
        let (tx, mut rx) = oneshot::channel();
        let tx = Arc::new(std::sync::Mutex::new(Some(tx)));

        let client = crate::client::get_client();
        client
            .user_stats()
            .request_global_stats(self.history_days, move |result| {
                let tx = tx.lock().unwrap().take().unwrap();
                let _ = tx.send(result);
            });

        // Run callbacks until we get the result
        loop {
            client.run_callbacks();
            if let Ok(result) = rx.try_recv() {
                return match result {
                    Ok(game_id) => Ok(game_id.raw()),
                    Err(_) => Err(Error::from_reason("Failed to request global stats")),
                };
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(BigInt::from(output))
    }
}
