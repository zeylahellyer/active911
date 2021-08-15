//! Requests for the Active911 API.

use crate::model::AlarmResponse;
use hyper::{
    body,
    client::{Client, HttpConnector},
    Body, Uri,
};
use hyper_rustls::HttpsConnector;
use std::{
    error::Error,
    fmt::{Display, Error as FmtError, Formatter},
    str::{self, FromStr},
};

#[cfg(not(any(feature = "rustls-native-certs", feature = "webpki-roots")))]
compile_error!("feature `rustls-native-certs` or `webpki-roots` must be chosen");

/// Base URL to the emitted JavaScript.
///
/// Must be appended to with a query parameter key with no value, in the form of
/// `?key`.
const BASE_URL: &str = "https://access.active911.com/interface/js.php";

/// Retrieving an agency's recent alarms failed.
#[derive(Debug)]
pub struct AlarmError {
    kind: AlarmErrorType,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl AlarmError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &AlarmErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (AlarmErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }
}

impl Display for AlarmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        f.write_str("")
    }
}

impl Error for AlarmError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Reason that an [`AlarmError`] occurred.
#[derive(Debug)]
pub enum AlarmErrorType {
    /// Response body is not UTF-8 valid.
    BodyNotUtf8,
    /// Response body could not be chunked, likely due to a network issue.
    Chunking,
    /// Response body could not be deserialized as recognized JSON.
    Deserializing,
    /// Expected JSON content is missing from the response body document.
    JsonMissing,
    /// Provided API key causes an invalid URI.
    KeyInvalid,
    /// Request could not be sent, likely due to a network issue.
    SendingRequest,
}

/// Retrieve the five most recent alarms.
///
/// # Errors
///
/// Returns an [`AlarmErrorType::BodyNotUtf8`] error type if the response body
/// is not UTF-8 valid.
///
/// Returns an [`AlarmErrorType::Chunking`] error type if the response body
/// could not be chunked, likely due to a network issue.
///
/// Returns an [`AlarmErrorType::Deserializing`] error type if the response
/// body's JSON content could not be deserialized properly.
///
/// Returns an [`AlarmErrorType::JsonMissing`] error type if the expected JSON
/// is missing from the response body.
///
/// Returns an [`AlarmErrorType::KeyInvalid`] error type if the given API key is
/// invalid and caused an invalid URI to be formed.
///
/// Returns an [`AlarmErrorType::SendingRequest`] error type if the request
/// could not be sent, likely due to a network issue.
pub async fn alarms(key: &str) -> Result<AlarmResponse, AlarmError> {
    let formatted_url = format!("{}?{}", BASE_URL, key);
    let uri = Uri::from_str(&formatted_url).map_err(|_| AlarmError {
        kind: AlarmErrorType::KeyInvalid,
        source: None,
    })?;

    let client = client();

    let res = client.get(uri).await.map_err(|source| AlarmError {
        kind: AlarmErrorType::SendingRequest,
        source: Some(Box::new(source)),
    })?;

    let bytes = body::to_bytes(res)
        .await
        .map_err(|source| AlarmError {
            kind: AlarmErrorType::Chunking,
            source: Some(Box::new(source)),
        })?
        .as_ref()
        .to_vec();

    let document = str::from_utf8(&bytes).map_err(|source| AlarmError {
        kind: AlarmErrorType::BodyNotUtf8,
        source: Some(Box::new(source)),
    })?;
    let json = find_json(document).ok_or(AlarmError {
        kind: AlarmErrorType::JsonMissing,
        source: None,
    })?;

    serde_json::from_str(json).map_err(|source| AlarmError {
        kind: AlarmErrorType::Deserializing,
        source: Some(Box::new(source)),
    })
}

/// Create a new client with an HTTPS connector according to the enabled feature
/// flag.
fn client() -> Client<HttpsConnector<HttpConnector>, Body> {
    #[cfg(feature = "rustls-native-certs")]
    let connector = HttpsConnector::with_native_roots();

    #[cfg(not(feature = "rustls-native-certs"))]
    let connector = HttpsConnector::with_webpki_roots();

    Client::builder().build(connector)
}

/// Find the JSON with the information in the JavaScript document and slice it
/// out.
fn find_json(input: &str) -> Option<&str> {
    const SETTER: &str = "a91.data=";

    input.get(input.find(SETTER)? + SETTER.len()..)
}

#[cfg(test)]
mod tests {
    use super::{AlarmError, AlarmErrorType};
    use static_assertions::assert_impl_all;
    use std::{error::Error, fmt::Debug};

    assert_impl_all!(AlarmError: Error, Send, Sync);
    assert_impl_all!(AlarmErrorType: Debug, Send, Sync);

    #[test]
    fn test_find_json() {
        const EXPECTED: &str = r#"{"result":"success","message":\n"#;
        const INPUT: &str = r#"};

        a91.data={"result":"success","message":\n"#;

        assert_eq!(Some(EXPECTED), super::find_json(INPUT));
    }
}
