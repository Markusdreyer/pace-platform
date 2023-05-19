use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Name {
    /// "Tobias Goulden"
    first: String,

    /// "Schultz"
    last: Option<String>,

    /// "Tobias G.S"
    display: String,
}

impl Name {
    fn new(&mut self, first: String, last: Option<String>) -> Name {
        let mut name = Name {
            first,
            last,
            display: String::new(),
        };

        self.update_display();

        name
    }

    fn set_first(&mut self, first: String) {
        self.first = first;
        self.update_display();
    }

    fn set_last(&mut self, last: Option<String>) {
        self.last = last;
        self.update_display();
    }

    fn update_display(&mut self) {
        match &self.last {
            Some(last) => {
                self.display = format!("{} {}", self.first, self.last.unwrap_or_default());
            }

            None => {
                self.display = self.first.clone();
            }
        }
    }
}

#[derive(Debug, Serialize)]
enum ImageSize {
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

#[derive(Debug, Serialize)]
struct ImageUrl {
    /// url to the image in it's original size
    original: String,
    sizes: Vec<(ImageSize, String)>,
}

impl ImageUrl {
    fn new(original: String) -> Self {
        // @todo parse and format https://cdn/:asset_id/:size's onto the each size
        Self {
            original,
            sizes: Vec::new(),
        }
    }

    fn with_size(mut self, size: ImageSize, url: String) -> Self {
        self.sizes.push((size, url));
        self
    }
}

#[derive(Debug, Serialize)]
struct Image {
    /// reference to the storage asset where the image is stored
    asset_id: Option<String>,

    /// url to the image in different sizes
    url: ImageUrl,

    /// for retards
    alt_text: Option<String>,
}

impl Image {
    fn new(original_url: String, asset_id: Option<String>, alt_text: Option<String>) -> Image {
        Image {
            asset_id: None,
            url: ImageUrl::new(original_url.to_string()),
            alt_text,
        }
    }

    fn set_asset_id(&mut self, asset_id: String) {
        self.asset_id = Some(asset_id);
        self
    }

    fn set_url(&mut self, url: String) {
        self.url = url;
        self
    }

    fn set_alt_text(&mut self, first: String) {
        self.first = first;
        self
    }
}

#[derive(Debug, Serialize)]
pub struct User {
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    name: Option<Name>,
    picture: Option<Image>,
    is_online: bool,
    last_online: Option<DateTime<Utc>>,
}

impl User {
    fn new() -> Self {
        let now = Utc::now();

        Self {
            created_at: now,
            updated_at: now,
            name: None,
            picture: None,
            is_online: false,
            last_online: None,
        }
    }

    fn set_name(&mut self, name: Name) {
        self.name = Some(name);
        self
    }

    fn set_picture(&mut self, picture: Image) {
        self.picture = Some(picture);
        self
    }

    fn set_is_online(&mut self, is_online: Boolean) {
        self.is_online = is_online;
        self.set_last_online(Utc::now());
        self
    }

    fn set_last_online(&mut self, last_online: DateTime<Utc>) {
        self.last_online = Some(last_online);
        self
    }
}
