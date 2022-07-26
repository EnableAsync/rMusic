use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::command;
use unm_engine::interface::Engine;
use unm_engine_kuwo::KuwoEngine;
use unm_types::{Context, RetrievedSongInfo, Song, SongSearchInformation};
// use unm_engine::executor::Executor;

#[derive(Serialize, Deserialize)]
pub struct MusicInfo {
    song: SongSearchInformation,
    retrieved: RetrievedSongInfo,
}

#[command]
pub async fn search_music(name: String) -> Result<Vec<MusicInfo>, String> {
    let engines: [Arc<dyn Engine + Send + Sync>; 1] = [
        Arc::new(KuwoEngine),
    ];
    let song = Song::builder().name(name).build();
    let mut search_result = Vec::new();
    for engine in engines.iter() {
        let context = Context::default();
        if let Ok(Some(song)) = engine.search(&song, &context).await {
            if let Ok(retrieved) = engine.retrieve(&song.identifier, &context).await {
                search_result.push(MusicInfo { song, retrieved });
            } else {
                log::error!("retrieve error: {:#?}", engine.retrieve(&song.identifier, &context).await.err());
            }
        } else {
            log::error!("search error: {:#?}", engine.search(&song, &context).await.err());
        }
    }

    log::debug!("result len: {}", search_result.len());

    if search_result.is_empty() {
        return Err("未搜索到歌曲".to_string());
    }

    Ok(search_result)
}
