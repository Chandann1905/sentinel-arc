use crate::types::{JsonRpcRequest, JsonRpcResponse};
use serde_json::Value;
use std::collections::HashMap;
use std::io;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::{Mutex, mpsc};
use tokio::task::JoinHandle;

pub struct StdioTransport;

impl StdioTransport {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StdioTransport {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn read_requests<F, Fut>(handler: F) -> io::Result<()>
where
    F: Fn(JsonRpcRequest) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = Option<JsonRpcResponse>> + Send + 'static,
{
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin).lines();

    // MPSC channel for serializing stdout writes safely
    let (tx, mut rx) = mpsc::channel::<JsonRpcResponse>(100);

    // stdout writer task
    tokio::spawn(async move {
        let mut stdout = tokio::io::stdout();
        while let Some(response) = rx.recv().await {
            if let Ok(mut serialized) = serde_json::to_string(&response) {
                serialized.push('\n');
                let _ = stdout.write_all(serialized.as_bytes()).await;
                let _ = stdout.flush().await;
            }
        }
    });

    let handler_arc = Arc::new(handler);
    let tasks: Arc<Mutex<HashMap<Value, JoinHandle<()>>>> = Arc::new(Mutex::new(HashMap::new()));

    while let Ok(Some(line)) = reader.next_line().await {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match serde_json::from_str::<JsonRpcRequest>(line) {
            Ok(request) => {
                // Intercept cancellation notification
                if request.method == "notifications/cancelled" {
                    if let Some(params) = request.params.as_ref() {
                        if let Some(req_id) = params.get("requestId").cloned() {
                            let mut tasks_lock = tasks.lock().await;
                            if let Some(handle) = tasks_lock.remove(&req_id) {
                                handle.abort();
                            }
                        }
                    }
                    continue;
                }

                let handler_clone = handler_arc.clone();
                let tx_clone = tx.clone();
                let req_id_clone = request.id.clone();
                let req_id_outer = req_id_clone.clone();
                let tasks_clone = tasks.clone();

                let handle = tokio::spawn(async move {
                    if let Some(response) = handler_clone(request).await {
                        let _ = tx_clone.send(response).await;
                    }
                    // Cleanup task ID when done
                    if let Some(id) = req_id_clone {
                        tasks_clone.lock().await.remove(&id);
                    }
                });

                if let Some(id) = req_id_outer {
                    tasks.lock().await.insert(id, handle);
                }
            }
            Err(_) => {
                // Return parse error directly
                let response =
                    JsonRpcResponse::error(None, -32700, "Parse error".to_string(), None);
                let _ = tx.send(response).await;
            }
        }
    }

    Ok(())
}
