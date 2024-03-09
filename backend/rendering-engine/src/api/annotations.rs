use crate::api::common::*;
use tokio::task;

pub async fn annotations(
    Extension(AppState {
        current_image,
        generators,
        ..
    }): Extension<AppState>,
) -> Response {
    let Some(current_image) = current_image.lock().unwrap().clone() else {
        return log::<()>(
            StatusCode::BAD_REQUEST,
            "Image metadata must first be fetched before requesting tiles.",
            None,
        );
    };

    #[cfg(feature = "log-success")]
    log::<()>(
        StatusCode::ACCEPTED,
        &format!(
            "Received request for annotations of image: {:?}.",
            current_image.image_name
        ),
        None,
    );

    let Some(ref annotations_name) = current_image.annotations_name else {
        let resp = log::<()>(
            StatusCode::NOT_FOUND,
            &format!(
                "Image with name {} does not have annotations.",
                current_image.image_name
            ),
            None,
        );

        return resp;
    };

    let annotations_path = current_image.directory_path.join(annotations_name);

    // TODO: Remove hardcoding
    let generator_name = "TIAToolbox";

    match task::spawn_blocking(move || {
        let binding = generators.lock().unwrap();
        let generator = match binding.get(generator_name) {
            Some(generator) => generator,
            None => {
                let resp = log::<()>(
                    StatusCode::BAD_REQUEST,
                    &format!(
                        "Annotation generator with name {} does not exist.",
                        generator_name
                    ),
                    None,
                );

                return resp;
            }
        };

        match generator.read_annotations(&annotations_path) {
            Ok(annotations) => {
                #[cfg(feature = "log-success")]
                log::<()>(StatusCode::OK, "Successfully retrieved annotations.", None);

                Json(annotations).into_response()
            }
            Err(e) => {
                let resp = log(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to retrieve annotations.",
                    Some(e),
                );

                resp
            }
        }
    })
    .await
    {
        Ok(response) => response,
        Err(e) => {
            let resp = log(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to spawn blocking task.",
                Some(e),
            );

            resp
        }
    }
}
