use std::fs::File;
use std::io::Write;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;
use std::time::Instant;
use tokio::sync::watch;
use uuid::Uuid;
use crate::util::reqwest_client::REQWEST_CLIENT;
/// 下载文件并实时返回进度（百分比），支持 302 跳转，可取消。
///
/// # 参数
/// - `url`: 下载链接
/// - `save_path`: 保存路径（含文件名）
/// - `progress_callback`: 进度回调，参数为 (id, 百分比)
/// - `cancel_token`: 取消信号，收到 true 时中断下载
///
/// # 返回
/// Ok((id, ()))，id为本次下载的唯一标识符
///
/// # 错误
/// 下载失败、写入失败或被取消时返回错误
pub async fn download_with_progress<F>(
    url: &str,
    save_path: PathBuf,
    mut progress_callback: F,
    cancel_token: &watch::Receiver<bool>,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
where
    F: FnMut(String, f64, f64) + Send + 'static, // 新增速度参数
{
    let id = Uuid::new_v4().to_string();
    let client = {
        let guard = REQWEST_CLIENT.lock().await;
        match &*guard {
            Some(c) => c.clone(),
            None => return Err("HTTP客户端未初始化".into()),
        }
    };
    if let Err(_) = catch_unwind(AssertUnwindSafe(|| {
        progress_callback(id.clone(), -1.0, 0.0)
    })) {
        return Err("进度回调发生 panic".into());
    }
    // 在发送 HTTP 请求前检查是否已取消
    if *cancel_token.borrow() {
        return Err("canceled".into());
    }
    let resp = match client.get(url).send().await {
        Ok(r) => r,
        Err(e) => return Err(format!("请求发送失败: {}", e).into()),
    };
    if !resp.status().is_success() {
        return Err(format!("下载失败，状态码: {}", resp.status()).into());
    }
    let total_size = resp.content_length();
    // 自动创建父目录
    if let Some(parent) = save_path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            return Err(format!("创建文件夹失败: {}", e).into());
        }
    }
    let mut file = match File::create(&save_path) {
        Ok(f) => f,
        Err(e) => return Err(format!("文件创建失败: {}", e).into()),
    };
    let mut stream = resp.bytes_stream();
    let mut downloaded: u64 = 0;
    let mut avg_speed = 0.0f64; // 平滑速度
    let mut last_speed_time = Instant::now();
    let mut last_speed_downloaded = 0u64;
    let mut last_reported_speed = 0.0f64;
    let speed_interval = std::time::Duration::from_millis(500);
    use futures_util::StreamExt;

    while let Some(chunk) = match stream.next().await {
        Some(Ok(c)) => Some(c),
        Some(Err(e)) => return Err(format!("下载数据块失败: {}", e).into()),
        None => None,
    } {
        if *cancel_token.borrow() {
            return Err("canceled".into());
        }
        if let Err(e) = file.write_all(&chunk) {
            return Err(format!("写入文件失败: {}", e).into());
        }
        downloaded += chunk.len() as u64;
        let now = Instant::now();
        // 仅每隔 speed_interval 统计一次速度
        if now.duration_since(last_speed_time) >= speed_interval {
            let bytes = downloaded - last_speed_downloaded;
            let secs = now.duration_since(last_speed_time).as_secs_f64();
            let speed = if secs > 0.0 {
                bytes as f64 / secs / 1024.0
            } else {
                0.0
            };
            // 平滑
            avg_speed = if avg_speed == 0.0 {
                speed
            } else {
                avg_speed * 0.7 + speed * 0.3
            };
            last_speed_time = now;
            last_speed_downloaded = downloaded;
            last_reported_speed = avg_speed;
        }
        let cb_result = if let Some(size) = total_size {
            let percent = (downloaded as f64 / size as f64) * 100.0;
            catch_unwind(AssertUnwindSafe(|| {
                progress_callback(id.clone(), percent.min(100.0), last_reported_speed)
            }))
        } else {
            catch_unwind(AssertUnwindSafe(|| {
                progress_callback(id.clone(), -1.0, last_reported_speed)
            }))
        };
        if cb_result.is_err() {
            return Err("进度回调发生 panic".into());
        }
    }
    if let Err(e) = file.flush() {
        return Err(format!("文件刷新失败: {}", e).into());
    }
    if let Err(_) = catch_unwind(AssertUnwindSafe(|| {
        progress_callback(id.clone(), 100.0, 0.0)
    })) {
        return Err("进度回调发生 panic".into());
    }
    Ok(id)
}
