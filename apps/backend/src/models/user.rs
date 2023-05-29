use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::Image;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: Option<Name>,
    pub picture: Option<Image>,
    pub last_online: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(user: ApiUserCreateRequest) -> Self {
        let now = Utc::now();

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            name: user.name,
            picture: user.picture,
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

    pub fn set_last_online(mut self, last_online: DateTime<Utc>) -> Self {
        self.last_online = Some(last_online);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ApiUserCreateRequest {
    #[validate]
    name: Option<Name>,

    #[validate]
    picture: Option<Image>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Name {
    /// "Tobias Goulden"
    #[validate(length(min = 3, max = 50))]
    first: String,

    /// "Schultz"
    #[validate(length(min = 3, max = 50))]
    last: Option<String>,

    /// "Tobias G.S"
    display: Option<String>,
}

impl Name {
    pub fn new(first: String, last: Option<String>) -> Name {
        let mut name = Name {
            first,
            last,
            display: None,
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
                self.display = Some(format!("{} {}", self.first, last));
            }

            None => {
                self.display = Some(self.first.clone());
            }
        }
    }
}
