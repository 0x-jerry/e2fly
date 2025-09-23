use futures::StreamExt;
use reqwest::IntoUrl;
use std::{
    env, fs, io::Write, path::{self, PathBuf}
};
use tauri::{AppHandle, Emitter, EventTarget, Manager, Runtime};

use crate::conf::AppConfigState;

pub async fn update_dat_files<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    let conf = {
        let app_conf_state = app.state::<AppConfigState>();
        let app_conf_state = app_conf_state.lock().unwrap();
        app_conf_state.conf.clone()
    };

    let xray_bin_path = conf.v2_fly.bin.clone();

    if xray_bin_path.is_empty() {
        return Err("Binary path is empty".to_string());
    }

    let bin_path =
        which::which(xray_bin_path.clone()).unwrap_or(PathBuf::from(xray_bin_path.clone()));

    let bin_path = bin_path.canonicalize().map_err(|e| e.to_string())?;

    let bin_dir = bin_path.parent().unwrap();

    let proxy_conf = if conf.active.enabled && conf.v2_fly.http.enabled {
        let http_proxy = format!(
            "http://{}:{}",
            conf.v2_fly.http.address, conf.v2_fly.http.port
        );
        let proxy = reqwest::Proxy::https(http_proxy).unwrap();

        Some(proxy)
    } else {
        None
    };

    let geoip_dat_url =
        "https://cdn.jsdelivr.net/gh/Loyalsoldier/v2ray-rules-dat@release/geoip.dat";
    let geosite_dat_url =
        "https://cdn.jsdelivr.net/gh/Loyalsoldier/v2ray-rules-dat@release/geosite.dat";

    let geoip_file = bin_dir.join("geoip.dat");
    let geosite_file = bin_dir.join("geosite.dat");

    download_to_file(
        geoip_dat_url,
        geoip_file,
        proxy_conf.clone(),
        Some(|progress| {
            app.emit_filter("download-progress", progress, |t| match t {
                EventTarget::WebviewWindow { label } => label == "main",
                _ => false,
            })
            .unwrap();
        }),
    )
    .await
    .map_err(|e| e.to_string())?;

    download_to_file(
        geosite_dat_url,
        geosite_file,
        proxy_conf,
        Some(|progress| {
            app.emit_filter("download-progress", progress, |t| match t {
                EventTarget::WebviewWindow { label } => label == "main",
                _ => false,
            })
            .unwrap();
        }),
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

async fn download_to_file<U, P>(
    url: U,
    dest_file: P,
    proxy: Option<reqwest::Proxy>,
    progress_fn: Option<impl Fn(DownloadProgress) + Copy>,
) -> Result<(), reqwest::Error>
where
    U: IntoUrl,
    P: AsRef<path::Path>,
{
    let client = reqwest::Client::builder();

    let client = match proxy {
        Some(proxy) => client.proxy(proxy),
        None => client,
    };

    let client = client.build()?;

    let response = client.get(url).send().await?;

    let mut progress_data = DownloadProgress {
        total: response.content_length().unwrap_or(0),
        name: response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .unwrap_or("unknown")
            .to_string(),
        ..Default::default()
    };

    let dest_filename = PathBuf::from(dest_file).file_name().unwrap();
    let temp_file_path = env::temp_dir().join(dest_filename);
    let mut file_handler = fs::File::create(temp_file_path).unwrap();
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        match item {
            Ok(bytes) => {
                progress_data.downloaded += bytes.len() as u64;

                if let Some(progress_fn) = progress_fn {
                    progress_fn(progress_data.clone())
                }

                file_handler.write_all(&bytes).unwrap();
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    file_handler.flush().unwrap();

    fs::copy(temp_file_path, dest_file).expect("Copy dat file failed");
    let _ = fs::remove_file(temp_file_path);

    Ok(())
}

#[derive(Debug, Default, Clone, Serialize)]
struct DownloadProgress {
    pub name: String,
    pub total: u64,
    pub downloaded: u64,
}
