use std::io::Read;

/// Read a document of a specific type from a collection
///
/// ## Arguments
/// * `auth` The authentication token
/// * `path` The document path / collection; For example `my_collection` or `a/nested/collection`
/// * `document_id` The document id. Make sure that you do not include the document id to the path argument.
pub fn read<T>(auth: &impl FirebaseAuthBearer, path: &str, document_id: impl AsRef<str>) 
    -> Result<T>
where
    for<'b> T: Deserialize<'b>,
{
    let document_name = document_name(&auth.project_id(), path, document_id);
    read_by_name(auth, &document_name)
}