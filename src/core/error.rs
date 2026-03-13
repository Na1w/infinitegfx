use thiserror::Error;

/// Standard error type for all graphics operations.
#[derive(Error, Debug)]
pub enum GfxError {
    #[error("Failed to request GPU adapter")]
    AdapterNotFound,
    #[error("Failed to request GPU device: {0}")]
    DeviceRequestFailed(#[from] wgpu::RequestDeviceError),
    #[error("Failed to create surface: {0}")]
    SurfaceError(String),
    #[error("Incompatible surface: {0}")]
    IncompatibleSurface(String),
    #[error("Resource initialization failed: {0}")]
    InitFailed(String),
}

/// Specialized Result type for graphics operations.
pub type GfxResult<T> = Result<T, GfxError>;
