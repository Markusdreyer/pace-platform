use std::fmt;

use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Name {
    /// "Tobias Goulden"
    first: String,

    /// "Schultz"
    last: Option<String>,

    /// "Tobias G.S"
    display: String,
}

impl Name {
    pub fn new(first: String, last: Option<String>) -> Name {
        let mut name = Name {
            first,
            last,
            display: String::new(),
        };

        name.update_display();

        name
    }

    pub fn set_first(&mut self, first: String) {
        self.first = first;
        self.update_display();
    }

    pub fn set_last(&mut self, last: Option<String>) {
        self.last = last;
        self.update_display();
    }

    fn update_display(&mut self) {
        match &self.last {
            Some(last) => {
                self.display = format!("{} {}", self.first, last.to_string());
            }

            None => {
                self.display = self.first.clone();
            }
        }
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display)
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
pub struct ImageUrl {
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
pub struct Image {
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

    fn set_asset_id(&mut self, asset_id: String) -> &mut Image {
        self.asset_id = Some(asset_id);
        self
    }

    fn set_url(&mut self, url: ImageUrl) -> &mut Image {
        self.url = url;
        self
    }

    fn set_alt_text(&mut self, first: String) -> &mut Image {
        self.alt_text = Some(first);
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
    pub fn new() -> Self {
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

    pub fn set_name(&mut self, name: Name) -> &mut User {
        self.name = Some(name);
        self
    }

    pub fn set_picture(&mut self, picture: Image) -> &mut User {
        self.picture = Some(picture);
        self
    }

    pub fn set_is_online(&mut self, is_online: bool) -> &mut User {
        self.is_online = is_online;
        self.set_last_online(Utc::now());
        self
    }

    pub fn set_last_online(&mut self, last_online: DateTime<Utc>) -> &mut User {
        self.last_online = Some(last_online);
        self
    }
}
