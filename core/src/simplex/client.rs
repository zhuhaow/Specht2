use crate::{
    io::Io,
    simplex::{io::into_io, Config, SimplexError},
    Endpoint, Result,
};
use http::Request;
use tokio_tungstenite::client_async;

pub async fn connect<I: Io>(
    io: I,
    endpoint: &Endpoint,
    config: &Config,
    host: String,
) -> Result<impl Io + 'static> {
    let request = Request::builder()
        .uri(format!("{}/{}", host, config.path))
        .header(&config.secret_header.0, &config.secret_header.1)
        .header("Simplex Connect", endpoint.to_string())
        .body(())
        .map_err(Into::<SimplexError>::into)?;

    let (stream, _response) = client_async(request, io).await?;

    Ok(into_io(stream))
}
