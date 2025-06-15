use common::recipe::RecipeImage;
use uuid::Uuid;
use web_sys::File;

use crate::request::get;

pub mod recipe_info;
pub mod recipe_ingredients;
pub mod recipe_steps;

pub async fn try_upload_image(file: Option<File>) -> Result<Option<Uuid>, anyhow::Error> {
    let Some(file) = file else {
        return Ok(None);
    };

    let image = match get("/api/recipes/image").send().await {
        Ok(res) if res.ok() => res.json::<RecipeImage>().await?,
        _ => {
            return Err(anyhow::anyhow!("Couldn't upload file"));
        }
    };

    reqwasm::http::Request::put(&image.url)
        .body(file.value_of())
        .send()
        .await?;

    Ok(Some(image.id))
}
