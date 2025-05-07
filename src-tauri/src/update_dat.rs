use futures::StreamExt;
use reqwest::IntoUrl;
use std::{fs, io::Write, path};
use tauri::{AppHandle, Emitter, EventTarget, Manager, Runtime};

use crate::conf::AppConfigState;

pub async fn update_dat_files<R: Runtime>(app: AppHandle<R>) -> Result<(), reqwest::Error> {
    let conf = {
        let app_conf_state = app.state::<AppConfigState>();
        let app_conf_state = app_conf_state.lock().unwrap();
        app_conf_state.conf.clone()
    };

    let xray_bin_path = conf.v2_fly.bin.clone();

    if !fs::exists(xray_bin_path.clone()).unwrap_or(false) {
        return Ok(());
    }

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

    let bin_path = path::PathBuf::from(xray_bin_path.clone());
    let bin_dir = bin_path.parent().unwrap();
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
    .await?;

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
    .await?;

    Ok(())
}

async fn download_to_file<U, P>(
    url: U,
    file: P,
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

    let mut file = fs::File::create(file).unwrap();
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        match item {
            Ok(bytes) => {
                progress_data.downloaded += bytes.len() as u64;

                if let Some(progress_fn) = progress_fn {
                    progress_fn(progress_data.clone())
                }

                file.write_all(&bytes).unwrap();
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    file.flush().unwrap();

    Ok(())
}

#[derive(Debug, Default, Clone, Serialize)]
struct DownloadProgress {
    pub name: String,
    pub total: u64,
    pub downloaded: u64,
}
