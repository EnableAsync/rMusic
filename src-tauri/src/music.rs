use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::command;
use unm_engine::interface::Engine;
// use unm_engine::executor::Executor;
use unm_engine_bilibili::BilibiliEngine;
use unm_engine_kugou::KugouEngine;
use unm_engine_kuwo::KuwoEngine;
use unm_engine_migu::MiguEngine;
use unm_engine_joox::JooxEngine;
use unm_types::{Context, RetrievedSongInfo, Song, SongSearchInformation};

#[derive(Serialize, Deserialize)]
pub struct MusicInfo {
    song: SongSearchInformation,
    retrieved: RetrievedSongInfo,
}

#[command]
pub async fn search_music(name: String) -> Result<Vec<MusicInfo>, String> {
    let engines: [Arc<dyn Engine + Send + Sync>; 5] = [
        Arc::new(MiguEngine),
        Arc::new(KuwoEngine),
        Arc::new(KugouEngine),
        Arc::new(BilibiliEngine),
        Arc::new(JooxEngine),
    ];
    let song = Song::builder().name(name).build();
    let context = Context::default();
    let mut search_result = Vec::new();
    for engine in engines.iter() {
        if let Ok(Some(song)) = engine.search(&song, &context).await {
            if let Ok(retrieved) = engine.retrieve(&song.identifier, &context).await {
                search_result.push(MusicInfo { song, retrieved });
            }
        }
    }

    if search_result.is_empty() {
        return Err("未搜索到歌曲".to_string());
    }

    Ok(search_result)
}
