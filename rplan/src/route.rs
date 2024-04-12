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

fn write_to_file(buf: Bytes) -> error::Result<(File, String)> {
    let mut hasher = Hasher::new();
    hasher.update_rayon(&buf);

    let file_hash_buf = hasher.finalize();
    let file_hash = format!("{file_hash_buf}");

    let mut temp_file = tempfile::tempfile()?;
    temp_file.write_all(&buf)?;

    Ok((temp_file, file_hash))
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
