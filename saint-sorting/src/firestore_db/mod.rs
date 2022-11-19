pub mod documents;
pub mod errors;
pub mod dto;
pub mod firebase_rest_to_rust;

/// Authentication trait.
///
/// This trait is implemented by [`crate::sessions`].
///
/// Firestore document methods in [`crate::documents`] expect an object that implements this `FirebaseAuthBearer` trait.
///
/// Implement this trait for your own data structure and provide the Firestore project id and a valid access token.
pub trait FirebaseAuthBearer {
    // Return the project ID. This is required for the firebase REST API.
    fn project_id(&self) -> &str;
    
    // An access token. If a refresh token is known and the access token expired,
    // the implementation should try to refresh the access token before returning.
    fn access_token(&self) -> String;
    
    // The access token, unchecked. Might be expired or in other ways invalid.
    fn access_token_unchecked(&self) -> String;

    // The reqwest http client.
    // The `Client` holds a connection pool internally, so it is advised that it is reused for multiple, successive connections.
    // fn client(&self) -> &reqwest::blocking::Client;

    // The reqwest http client.
    // The `Client` holds a connection pool internally, so it is advised that it is reused for multiple, successive connections.
    // fn client_async(&self) -> &reqwest::Client;
}
