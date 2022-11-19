//! # Firestore Document Access
//!
//! Interact with Firestore documents.
//! Please check the root page of this documentation for examples.

use super::dto;
use super::errors::{extract_google_api_error, FirebaseError, Result};
use super::firebase_rest_to_rust::{document_to_pod, pod_to_document};
// use super::FirebaseAuthBearer;

use serde::{Deserialize, Serialize};
use std::path::Path;


// pub mod read;
pub mod write;

// pub use read::*;
pub use write::*;

#[inline]
fn firebase_url_query(v1: &str) -> String {
    format!(
        "https://firestore.googleapis.com/v1/projects/{}/databases/(default)/documents:runQuery",
        v1
    )
}

#[inline]
fn firebase_url_base(v1: &str) -> String {
    format!("https://firestore.googleapis.com/v1/{}", v1)
}

#[inline]
fn firebase_url_extended(v1: &str, v2: &str, v3: &str) -> String {
    format!(
        "https://firestore.googleapis.com/v1/projects/{}/databases/(default)/documents/{}/{}",
        v1, v2, v3
    )
}

#[inline]
fn firebase_url(v1: &str, v2: &str) -> String {
    format!(
        "https://firestore.googleapis.com/v1/projects/{}/databases/(default)/documents/{}?",
        v1, v2
    )
}