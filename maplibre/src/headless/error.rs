use thiserror::Error;

#[derive(Error, Debug)]
pub enum HeadlessRenderError {
    #[error("error while rendering to png")]
    WritePng(#[from] png::EncodingError),
    #[error("could not create file to save as an image")]
    CreateImageFileFailed(#[from] std::io::Error),
}
