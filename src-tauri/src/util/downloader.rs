use reqwest::Client;
use std::fs::File;
use std::io::Write;
use std::panic::{catch_unwind, AssertUnwindSafe};
use tokio::sync::watch;
use uuid::Uuid;

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
    save_path: &str,
    mut progress_callback: F,
    cancel_token: &watch::Receiver<bool>,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
where
    F: FnMut(String, f64) + Send + 'static,
{
    let id = Uuid::new_v4().to_string();
    let client = match Client::builder().redirect(reqwest::redirect::Policy::limited(10)).build() {
        Ok(c) => c,
        Err(e) => return Err(format!("HTTP客户端初始化失败: {}", e).into()),
    };
    let resp = match client.get(url).send().await {
        Ok(r) => r,
        Err(e) => return Err(format!("请求发送失败: {}", e).into()),
    };
    if !resp.status().is_success() {
        return Err(format!("下载失败，状态码: {}", resp.status()).into());
    }
    let total_size = resp.content_length();
    let mut file = match File::create(save_path) {
        Ok(f) => f,
        Err(e) => return Err(format!("文件创建失败: {}", e).into()),
    };
    let mut stream = resp.bytes_stream();
    let mut downloaded: u64 = 0;
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
        let cb_result = if let Some(size) = total_size {
            let percent = (downloaded as f64 / size as f64) * 100.0;
            catch_unwind(AssertUnwindSafe(|| progress_callback(id.clone(), percent.min(100.0))))
        } else {
            catch_unwind(AssertUnwindSafe(|| progress_callback(id.clone(), 0.0)))
        };
        if cb_result.is_err() {
            return Err("进度回调发生 panic".into());
        }
    }
    if let Err(e) = file.flush() {
        return Err(format!("文件刷新失败: {}", e).into());
    }
    if let Err(_) = catch_unwind(AssertUnwindSafe(|| progress_callback(id.clone(), 100.0))) {
        return Err("进度回调发生 panic".into());
    }
    Ok(id)
}


