use std::error::Error as StdError;

#[derive(Debug)]
pub enum Error {
    /// The http connection failed
    HttpError(reqwest::Error),

    /// An attempt was made to upload a resource with size stored in field `.0`
    /// even though the maximum upload size is what is stored in field `.1`.
    UploadSizeLimitExceeded(u64, u64),

    /// Represents information about a request that was not understood by the server.
    /// Details are included.
    BadRequest(serde_json::Value),

    /// We needed an API key for authentication, but didn't obtain one.
    /// Neither through the authenticator, nor through the Delegate.
    MissingAPIKey,

    /// We required a Token, but didn't get one from the Authenticator
    MissingToken(Box<dyn StdError>),

    /// The delgate instructed to cancel the operation
    Cancelled,

    /// An additional, free form field clashed with one of the built-in optional ones
    FieldClash(&'static str),

    /// Shows that we failed to decode the server response.
    JsonDecodeError(String),

    /// Indicates an HTTP repsonse with a non-success status code
    Failure(reqwest::Response),

    /// An IO error occurred while reading a stream into memory
    Io(std::io::Error),
}
