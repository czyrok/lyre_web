use axum::response::Response;
use http::header;
use sitemap_generator::SitemapBuilder;

pub async fn sitemap_handler(builder: SitemapBuilder) -> Response {
    let sitemap_xml = builder.build().unwrap();

    let mut response = Response::new(axum::body::Body::from(sitemap_xml));

    response.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/xml"),
    );

    response
}
