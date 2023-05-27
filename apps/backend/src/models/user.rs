use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::Image;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub fn set_name(mut self, name: Name) -> Self {
        self.name = Some(name);
        self
    }

    #[allow(dead_code)]
    pub fn set_picture(mut self, picture: Image) -> Self {
        self.picture = Some(picture);
        self
    }

    #[allow(dead_code)]
    pub fn set_is_online(mut self, is_online: bool) -> User {
        self.is_online = is_online;
        
        self.set_last_online(Utc::now())
    }

    pub fn set_last_online(mut self, last_online: DateTime<Utc>) -> Self {
        self.last_online = Some(last_online);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[allow(dead_code)]
    pub fn set_first(&mut self, first: String) {
        self.first = first;
        self.update_display();
    }

    #[allow(dead_code)]
    pub fn set_last(&mut self, last: Option<String>) {
        self.last = last;
        self.update_display();
    }

    fn update_display(&mut self) {
        match &self.last {
            Some(last) => {
                self.display = format!("{} {}", self.first, last);
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
