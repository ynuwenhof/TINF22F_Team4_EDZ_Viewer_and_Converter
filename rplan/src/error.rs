use axum::extract::multipart::MultipartError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use handlebars::{RenderError, TemplateError};
use quick_xml::events::attributes::AttrError;
use std::fmt::{Display, Formatter};
use std::{error, io, result};
use tokio::sync::oneshot::error::RecvError;
use tokio::task::JoinError;
use tracing::error;
use url::ParseError;
use zip::result::ZipError;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Attr(AttrError),
    DateOutOfRange,
    Invalid7zSignature,
    InvalidJfifSignature,
    PathComponentDenied,
    Io(io::Error),
    Join(JoinError),
    Libarchive(libarchive::Error),
    MongoDb(mongodb::error::Error),
    Multipart(MultipartError),
    Recv(RecvError),
    Render(RenderError),
    Template(TemplateError),
    Url(ParseError),
    Xml(quick_xml::Error),
    XmlDe(quick_xml::DeError),
    Zip(ZipError),
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Attr(e) => e.fmt(f),
            Self::DateOutOfRange => write!(f, "date out of range"),
            Self::Invalid7zSignature => write!(f, "invalid 7z signature"),
            Self::InvalidJfifSignature => write!(f, "invalid JFIF signature"),
            Self::PathComponentDenied => write!(f, "path component denied"),
            Self::Io(e) => e.fmt(f),
            Self::Join(e) => e.fmt(f),
            Self::Libarchive(e) => e.fmt(f),
            Self::MongoDb(e) => e.fmt(f),
            Self::Multipart(e) => e.fmt(f),
            Self::Recv(e) => e.fmt(f),
            Self::Render(e) => e.fmt(f),
            Self::Template(e) => e.fmt(f),
            Self::Url(e) => e.fmt(f),
            Self::Xml(e) => e.fmt(f),
            Self::XmlDe(e) => e.fmt(f),
            Self::Zip(e) => e.fmt(f),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("{self:?}");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

impl From<AttrError> for Error {
    fn from(e: AttrError) -> Self {
        Self::Attr(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<JoinError> for Error {
    fn from(e: JoinError) -> Self {
        Self::Join(e)
    }
}

impl From<mongodb::error::Error> for Error {
    fn from(e: mongodb::error::Error) -> Self {
        Self::MongoDb(e)
    }
}

impl From<libarchive::Error> for Error {
    fn from(e: libarchive::Error) -> Self {
        Self::Libarchive(e)
    }
}

impl From<MultipartError> for Error {
    fn from(e: MultipartError) -> Self {
        Self::Multipart(e)
    }
}

impl From<RecvError> for Error {
    fn from(e: RecvError) -> Self {
        Self::Recv(e)
    }
}

impl From<RenderError> for Error {
    fn from(e: RenderError) -> Self {
        Self::Render(e)
    }
}

impl From<TemplateError> for Error {
    fn from(e: TemplateError) -> Self {
        Self::Template(e)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Self::Url(e)
    }
}

impl From<quick_xml::Error> for Error {
    fn from(e: quick_xml::Error) -> Self {
        Self::Xml(e)
    }
}

impl From<quick_xml::DeError> for Error {
    fn from(e: quick_xml::DeError) -> Self {
        Self::XmlDe(e)
    }
}

impl From<ZipError> for Error {
    fn from(e: ZipError) -> Self {
        Self::Zip(e)
    }
}
