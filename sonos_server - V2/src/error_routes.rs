use rocket::Request;

#[catch(404)]
pub async fn not_found(req: &Request<'_>) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[catch(500)]
pub async fn internal_error(req: &Request<'_>) -> String {
    format!("Sorry, an error occurred on the server. Error: '{}'.", req)
}
