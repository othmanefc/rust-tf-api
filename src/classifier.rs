use actix_web::web::Json;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

use tract_tensorflow::prelude::*;

use image;

pub struct Img {
    pub tensor: Tensor,
}

impl Img {
    pub fn new(path: String) -> Self {
        Self {
            tensor: Img::path_to_tensor(path),
        }
    }

    fn path_to_tensor(path: String) -> Tensor {
        let img = image::open(path).unwrap().to_rgb8();
        let resized =
            image::imageops::resize(&img, 224, 224, image::imageops::FilterType::Triangle);
        tract_ndarray::Array4::from_shape_fn((1, 224, 224, 3), |(_, y, x, c)| {
            resized[(x as _, y as _)][c] as f32 / 255.0
        })
        .into()
    }
}

fn predict(img: Img) -> TractResult<Option<(f32, i32)>> {
    let model = tract_tensorflow::tensorflow()
        .model_for_path("model.pb")?
        .with_input_fact(
            0,
            InferenceFact::dt_shape(f32::datum_type(), tvec!(1, 224, 224, 3)),
        )?
        .into_optimized()?
        .into_runnable()?;
    let prediction = model.run(tvec!(img.tensor))?;
    let a = prediction[0]
        .to_array_view::<f32>()?
        .iter()
        .cloned()
        .zip(1..)
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    println!("prediction: {:?}", a);
    Ok(a)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImgRequest {
    pub path: Option<String>,
}

impl ImgRequest {
    pub fn to_img(&self) -> Option<Img> {
        match &self.path {
            Some(path) => Some(Img::new(path.to_string())),
            None => None,
        }
    }
}

#[get("/classes")]
pub async fn get(img_req: Json<ImgRequest>) -> HttpResponse {
    let img = img_req.to_img().unwrap();
    let prediction = predict(img);
    match prediction {
        Ok(prediction) => HttpResponse::Ok()
            .content_type("application/json")
            .json(prediction.unwrap()),
        _ => HttpResponse::NoContent().await.unwrap(),
    }
}
