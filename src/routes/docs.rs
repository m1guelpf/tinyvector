use aide::{
	axum::{routing::get, ApiRouter},
	openapi::OpenApi,
};
use axum::{response::Html, Extension};
use axum_jsonschema::Json;

pub fn handler() -> ApiRouter {
	ApiRouter::new()
		.route("/docs", get(swagger))
		.route("/openapi.json", get(openapi_schema))
}

#[allow(clippy::unused_async)]
async fn openapi_schema(Extension(openapi): Extension<OpenApi>) -> Json<OpenApi> {
	Json(openapi)
}

#[allow(clippy::unused_async)]
async fn swagger() -> Html<String> {
	Html(SWAGGER_UI_TEMPLATE.replace("{:spec_url}", "/openapi.json"))
}

const SWAGGER_UI_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <title>Tinyvector Docs</title>
    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@5.10.5/swagger-ui.css">
    <script src="https://unpkg.com/swagger-ui-dist@5.10.5/swagger-ui-standalone-preset.js"></script>
    <script src="https://unpkg.com/swagger-ui-dist@5.10.5/swagger-ui-bundle.js"></script>
  </head>
  <body>
    <div id="swagger-ui"></div>
    <script>
      window.onload = function() {
        window.ui = SwaggerUIBundle({
          url: "{:spec_url}", dom_id: '#swagger-ui', deepLinking: true,
          presets: [SwaggerUIBundle.presets.apis, SwaggerUIStandalonePreset],
          plugins: [SwaggerUIBundle.plugins.DownloadUrl], layout: "StandaloneLayout"
        })
      }
    </script>
  </body>
</html>
"#;
