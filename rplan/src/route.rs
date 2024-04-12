use super::context::Context;
use super::error;
use super::model::Sample;
use super::queue::{Entry, Status, Task};
use axum::body::{Body, Bytes};
use axum::extract::{Multipart, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{extract, Json};
use blake3::Hasher;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Component;
use std::path::Path;
use time::serde::rfc3339;
use time::OffsetDateTime;
use tokio::fs;
use tokio::task;
use tokio_util::io::ReaderStream;
use url::Url;

pub async fn create_sample(
    State(ctx): State<Context>,
    mut multipart: Multipart,
) -> error::Result<Response> {
    let field = multipart.next_field().await?;
    let field = match field {
        None => return Ok(StatusCode::BAD_REQUEST.into_response()),
        Some(field) => field,
    };

    let file_name = match field.file_name() {
        None => return Ok(StatusCode::BAD_REQUEST.into_response()),
        Some(file_name) => file_name,
    }
    .to_string();

    let buf = field.bytes().await?;
    let file_size = buf.len();

    let (temp_file, file_hash) = task::spawn_blocking(move || write_to_file(buf)).await??;

    if let Some(sample) = ctx.db.find_sample(&file_hash).await? {
        let sample = SampleResponse::from(sample);
        return Ok((StatusCode::OK, Json(sample)).into_response());
    }

    let task = Task {
        status: Status::Pending,
        hash: file_hash,
        name: file_name,
        size: file_size,
        created: OffsetDateTime::now_utc(),
    };

    let entry = Entry {
        task,
        file: temp_file,
    };

    let task = ctx.queue.push_entry(entry).await?;

    let sample = SampleResponse::from(task);
    Ok((StatusCode::CREATED, Json(sample)).into_response())
}

pub async fn get_samples(State(ctx): State<Context>) -> error::Result<Response> {
    let samples: Vec<SampleResponse> = ctx
        .db
        .find_samples()
        .await?
        .into_iter()
        .map(SampleResponse::from)
        .collect();

    Ok((StatusCode::OK, Json(samples)).into_response())
}

pub async fn get_sample(
    State(ctx): State<Context>,
    extract::Path(sample_hash): extract::Path<String>,
) -> error::Result<Response> {
    let sample = ctx.db.find_sample(&sample_hash).await?;
    if let Some(sample) = sample {
        let sample = SampleResponse::from(sample);
        return Ok((StatusCode::OK, Json(sample)).into_response());
    }

    let task = ctx.queue.get_task(sample_hash).await;
    if let Some(task) = task {
        let sample = SampleResponse::from(task);
        return Ok((StatusCode::OK, Json(sample)).into_response());
    }

    Ok(StatusCode::NOT_FOUND.into_response())
}

State(ctx): State<Context>,
extract::Path(sample_hash): extract::Path<String>,
) -> error::Result<Response> {
let sample = ctx.db.find_sample(&sample_hash).await?;
if sample.is_none() {
    return Ok(StatusCode::NOT_FOUND.into_response());
}

let sample_path = ctx.cli.data_path.join(&sample_hash);
let dir_entries = list_dir(&sample_path).await?;

Ok((StatusCode::OK, Json(dir_entries)).into_response())
}

pub async fn get_blob_file(
    State(ctx): State<Context>,
    extract::Path((sample_hash, path)): extract::Path<(String, String)>,
) -> error::Result<Response> {
    use tokio::fs::File;

    let sample = ctx.db.find_sample(&sample_hash).await?;
    if sample.is_none() {
        return Ok(StatusCode::NOT_FOUND.into_response());
    }

    let path = Path::new(&path);
    let invalid_component = path
        .components()
        .any(|c| !matches!(c, Component::Normal(_)));

    if invalid_component {
        return Ok(StatusCode::NOT_FOUND.into_response());
    }

    let file_path = ctx.cli.data_path.join(&sample_hash).join(path);
    if file_path.is_dir() {
        let dir_entries = list_dir(&file_path).await?;
        return Ok((StatusCode::OK, Json(dir_entries)).into_response());
    }

    let file = match File::open(&file_path).await {
        Ok(file) => file,
        Err(_) => return Ok(StatusCode::NOT_FOUND.into_response()),
    };

    let content_type = mime_guess::from_path(&file_path)
        .first_raw()
        .map(|s| [(header::CONTENT_TYPE, s)]);

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Ok((StatusCode::OK, content_type, body).into_response())
}

fn write_to_file(buf: Bytes) -> error::Result<(File, String)> {
    let mut hasher = Hasher::new();
    hasher.update_rayon(&buf);

    let file_hash_buf = hasher.finalize();
    let file_hash = format!("{file_hash_buf}");

    let mut temp_file = tempfile::tempfile()?;
    temp_file.write_all(&buf)?;

    Ok((temp_file, file_hash))
}

async fn list_dir<P: AsRef<Path>>(path: P) -> error::Result<Vec<DirEntryResponse>> {
    let mut dir = fs::read_dir(path).await?;
    let mut dir_entries = Vec::new();

    while let Some(entry) = dir.next_entry().await? {
        let file_name = match entry.file_name().into_string() {
            Ok(file_name) => file_name,
            Err(_) => continue,
        };

        let file_type = entry.file_type().await?;
        if !file_type.is_dir() && !file_type.is_file() {
            continue;
        }

        let entry = DirEntryResponse {
            name: file_name,
            is_dir: file_type.is_dir(),
        };

        dir_entries.push(entry);
    }

    Ok(dir_entries)
}

#[derive(Serialize)]
pub struct SampleResponse {
    id: String,
    name: String,
    size: usize,
    #[serde(with = "rfc3339")]
    created: OffsetDateTime,
    #[serde(flatten)]
    status: StatusResponse,
}

impl From<Task> for SampleResponse {
    fn from(task: Task) -> Self {
        let status = match task.status {
            Status::Pending => StatusResponse::Pending,
            Status::Running => StatusResponse::Running,
            Status::Failed => StatusResponse::Failed,
        };

        Self {
            id: task.hash,
            name: task.name,
            size: task.size,
            created: task.created,
            status,
        }
    }
}

impl From<Sample> for SampleResponse {
    fn from(sample: Sample) -> Self {
        let completed = CompletedResponse {
            completed: sample.completed,
            expires: sample.expires,
            packages: sample.packages,
        };

        Self {
            id: sample.hash,
            name: sample.name,
            size: sample.size,
            created: sample.created,
            status: StatusResponse::Completed(completed),
        }
    }
}

#[derive(Serialize)]
pub struct DirEntryResponse {
    name: String,
    is_dir: bool,
}

#[derive(Serialize)]
#[serde(tag = "status", rename_all = "lowercase")]
enum StatusResponse {
    Pending,
    Running,
    Completed(CompletedResponse),
    Failed,
}

#[derive(Serialize)]
struct CompletedResponse {
    #[serde(with = "rfc3339")]
    completed: OffsetDateTime,
    #[serde(with = "rfc3339")]
    expires: OffsetDateTime,
    packages: usize,
}
