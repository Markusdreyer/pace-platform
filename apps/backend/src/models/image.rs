use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum ImageSize {
    Lqip,
    W50,
    W200,
    W400,
    W600,
    W800,
    W1200,
    W1800,
    W2500,
}

#[derive(Debug, Clone, Serialize, Validate, Deserialize)]
pub struct ImageUrl {
    /// url to the image in it's original size
    #[validate(url)]
    original: String,

    /// @todo implement
    sizes: Vec<(ImageSize, String)>,
}

impl ImageUrl {
    #[allow(dead_code)]

    fn new(original: String) -> Self {
        // @todo parse and format https://cdn/:asset_id/:size's onto the each size
        Self {
            original,
            sizes: Vec::new(),
        }
    }

    #[allow(dead_code)]
    fn with_size(mut self, size: ImageSize, url: String) -> Self {
        self.sizes.push((size, url));
        self
    }
}

#[derive(Debug, Clone, Serialize, Validate, Deserialize)]
pub struct Image {
    /// reference to the storage asset where the image is stored
    #[validate(length(min = 3, max = 255))]
    asset_id: Option<String>,

    /// url to the image in different sizes
    #[validate]
    url: ImageUrl,

    /// for retards
    #[validate(length(min = 3, max = 255))]
    alt_text: Option<String>,
}

impl Image {
    #[allow(dead_code)]
    fn new(original_url: String, _asset_id: Option<String>, alt_text: Option<String>) -> Image {
        Image {
            asset_id: None,
            url: ImageUrl::new(original_url),
            alt_text,
        }
    }

    #[allow(dead_code)]
    fn set_asset_id(&mut self, asset_id: String) -> &mut Image {
        self.asset_id = Some(asset_id);
        self
    }

    #[allow(dead_code)]
    fn set_url(&mut self, url: ImageUrl) -> &mut Image {
        self.url = url;
        self
    }

    #[allow(dead_code)]
    fn set_alt_text(&mut self, first: String) -> &mut Image {
        self.alt_text = Some(first);
        self
    }
}
