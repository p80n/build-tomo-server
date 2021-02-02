use rocket::response::content;

#[get("/healthz")]
pub fn healthz() -> content::Json<&'static str> {
    content::Json("{ 'ok': 'true' }")
}
