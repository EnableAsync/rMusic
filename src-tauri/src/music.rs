use std::{borrow::Cow, sync::Arc};
use unm_engine::executor::{Executor};
use unm_engine_migu::{MiguEngine, ENGINE_ID as MIGU_ENGINE_ID};
use unm_types::{Song, Context, SongSearchInformation};
use tauri::{command};

#[command]
pub async fn search_music(name: String) -> Result<SongSearchInformation, String> {
    let mut executor = Executor::new();
    let engine_id = Cow::from(MIGU_ENGINE_ID);
    executor.register(engine_id.clone(), Arc::new(MiguEngine));
    let song = Song::builder().name(name).build();
    let context = Context::default();
    let search_result = executor.search(&[engine_id], &song, &context).await.expect("search error");
    // let result = executor.retrieve(&search_result, &context).await.expect("retrieve error");
    return Ok(search_result)
}