use crate::coords::WorldTileCoords;
use crate::io::http::HttpClient;
use crate::style::source::TileAddressingScheme;
use thiserror::Error;

const MAPTILER_RASTER_SOURCE: Source = Source::new(
    "maptiler_raster",
    TileAddressingScheme::XYZ,
    SourceUrl::new(
        "https://api.maptiler.com/tiles/satellite-v2/{x}/{y}/{z}.jpg?key=qnePkfbGpMsLCi3KFBs3",
    ),
);

const TUERANTUER_VECTOR_SOURCE: Source = Source::new(
    "tuerantuer_vector",
    TileAddressingScheme::XYZ,
    SourceUrl::new("https://maps.tuerantuer.org/europe_germany/{x}/{y}/{z}.pbf"),
);

pub struct SourceUrl {
    url: String,
}

impl SourceUrl {
    pub fn new<U: Into<String>>(url: U) -> Self {
        Self { url: url.into() }
    }

    pub fn format(&self, coords: WorldTileCoords) -> String {
        self.url
            .replace("{x}", &coords.x.to_string())
            .replace("{y}", &coords.y.to_string())
            .replace("{z}", &coords.z.to_string())
    }
}

pub struct Source {
    name: String,
    addressing_scheme: TileAddressingScheme,
    // TODO mimetype
    url: SourceUrl,
}

impl Source {
    pub fn new<N: Into<String>>(
        name: N,
        addressing_scheme: TileAddressingScheme,
        url: SourceUrl,
    ) -> Self {
        Self {
            name: name.into(),
            addressing_scheme,
            url,
        }
    }
}

#[derive(Error, Debug)]
#[error("failed to fetch from source")]
pub struct SourceFetchError(#[source] pub Box<dyn std::error::Error>);

pub trait SourceFetchResult {
    fn as_bytes(&self) -> &[u8];
}

impl SourceFetchResult for Vec<u8> {
    fn as_bytes(&self) -> &[u8] {
        self.as_ref()
    }
}

/// Defines the different types of HTTP clients such as basic HTTP and Mbtiles.
/// More types might be coming such as S3 and other cloud http clients.
pub trait SourceClient {
    type Result: SourceFetchResult;

    async fn fetch(
        &self,
        source: &Source,
        coords: WorldTileCoords,
    ) -> Result<Self::Result, SourceFetchError>;
}

/// Gives access to the HTTP client which can be of multiple types,
/// see [crates::io::source_client::SourceClient]
#[derive(Clone)]
pub struct HttpSourceClient {
    inner_client: Box<dyn HttpClient>,
}

impl HttpSourceClient {
    pub fn new<HC: HttpClient>(http_client: HC) -> Self {
        Self {
            inner_client: http_client,
        }
    }
}

impl SourceClient for HttpSourceClient {
    type Result = Vec<u8>;

    async fn fetch(
        &self,
        source: &Source,
        coords: WorldTileCoords,
    ) -> Result<Self::Result, SourceFetchError> {
        self.inner_client.fetch(source.url.format(coords).as_str())
    }
}
