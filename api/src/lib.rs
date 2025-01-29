mod templates;

use actix_web::{
    get, Result,
};

use maud::Markup;

#[get("/")]
async fn index() -> Result<Markup> {
    Ok(templates::index::index())
}

#[get("/banned")]
async fn banned() -> Result<Markup> {
    Ok(templates::misc::banned())
}

#[get("/download")]
async fn download() -> Result<Markup> {
    Ok(templates::files::download())
}

pub async fn not_found() -> Result<Markup> {
    Ok(templates::misc::not_found())
}