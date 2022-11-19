use super::*;

/// This structure contains the document id of the written document.
#[derive(Serialize, Deserialize)]
pub struct WriteResult {
    ///
    pub create_time: Option<chrono::DateTime<chrono::Utc>>,
    pub update_time: Option<chrono::DateTime<chrono::Utc>>,
    pub document_id: String,
}

/// Write options. The default will overwrite a target document and not merge fields.
#[derive(Default)]
pub struct WriteOptions {
    /// If this is set instead of overwriting all fields of a target document, only the given fields will be merged.
    /// This only works if your document type has Option fields.
    /// The write will fail, if no document_id is given or the target document does not exist yet.
    pub merge: bool,
}

///
/// ## Arguments
/// * 'auth' The authentication token
/// * 'path' The document path / collection; For example "my_collection" or "a/nested/collection"
/// * 'document_id' The document id. Make sure that you do not include the document id in the path argument.
/// * 'document' The document
/// * 'options' Write options

pub fn write<T>(
    // auth: &impl FirebaseAuthBearer,
    path: &str,
    document_id: Option<impl AsRef<str>>,
    document: &T,
    options: WriteOptions,
) -> Result<WriteResult>
where
    T: Serialize,
{
    //Hard Code
    let test_project_id = "supersaintex-61278";

    let mut url = match document_id.as_ref() {
        Some(document_id) => firebase_url_extended(test_project_id, path, document_id.as_ref()),
        None => firebase_url(test_project_id, path),
    };

    //documentの型を変える.
    //https://github.com/davidgraeff/firestore-db-and-auth-rs/blob/master/src/firebase_rest_to_rust.rs
    let firebase_document = pod_to_document(&document)?;

    //既存のドキュメントに追加するとき
    if options.merge && firebase_document.fields.is_some() {
        let fields = firebase_document.fields.as_ref().unwrap().keys().join(",");
        url = format!("{}?currentDocument.exists=true&updateMask.fieldPaths={}", url, fields);
    }

    //新しくドキュメントを作るとき
    // let builder = if document_id.is_some() {
    //     auth.client().patch(&url)
    // } else {
    //     auth.client().post(&url)
    // };

    // let resp = builder
    //     .bearer_auth(auth.access_token().to_owned())
    //     .json(&firebase_document)
    //     .send()?;

    let resp = extract_google_api_error({
        document_id
            .as_ref()
            .and_then(|f| Some(f.as_ref().to_owned()))
            .or(Some(String::new()))
            .unwrap()
    })?;

    let result_document: dto::Document = resp.json()?;

    let document_id = document_id.to_string();
    // let document_id = Path::new(&result_document.name)
    //     .file_name()
    //     .ok_or_else(|| FirebaseError::Generic("Resulting documents 'name' field is not a valid path"))?
    //     .to_str()
    //     .ok_or_else(|| FirebaseError::Generic("No valid unicode in 'name' field"))?
    //     .to_owned();

    let create_time = match result_document.create_time {
        Some(f) => Some(
            chrono::DateTime::parse_from_rfc3339(&f)
                .map_err(|_| FirebaseError::Generic("Failed to parse rfc3339 date from 'create_time' field"))?
                .with_timezone(&chrono::Utc),
        ),
        None => None,
    };
    let update_time = match result_document.update_time {
        Some(f) => Some(
            chrono::DateTime::parse_from_rfc3339(&f)
                .map_err(|_| FirebaseError::Generic("Failed to parse rfc3339 date from 'update_time' field"))?
                .with_timezone(&chrono::Utc),
        ),
        None => None,
    };

    Ok(WriteResult {
        document_id,
        create_time,
        update_time,
    })
}