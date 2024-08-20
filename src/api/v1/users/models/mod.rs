use std::fmt;

use crate::schema::{user_roles, users::{self, password}};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};
use uuid::Uuid;
type DbConn = diesel::PgConnection;
use crate::utils::error;
use sha2::{Sha256, Digest};

#[derive(Queryable, Insertable, Debug)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub bio: Option<String>,
    pub dp_url: Option<String>,
}

#[derive(Queryable, Insertable, Debug)]
#[table_name = "user_roles"]
pub struct UserRoleXref {
    pub user_id: uuid::Uuid,
    pub role: Role,
}
impl UserRoleXref {
    pub fn insert_user_roles(
        value: &Vec<Self>,
        conn: &mut DbConn,
    ) -> Result<(), error::ServerError> {
        use crate::schema::user_roles;
        let _ = diesel::insert_into(user_roles::table)
            .values(value)
            .execute(conn)?;
        Ok(())
    }
}

impl User {
    pub fn new(email: String, pass: String, first_name: String, last_name: String) -> Self {
        // change email into smaller case

        // hash password using sha256 and strore here
        let email = email.to_lowercase();
        let pass = hash_password(&pass);
        Self {
            password: pass,
            first_name,
            last_name,
            id: uuid::Uuid::new_v4(),
            email: email,
            bio: None,
            dp_url: None,
        }
    }

    pub fn insert(&self, conn: &mut DbConn) -> Result<uuid::Uuid, error::ServerError> {
        // store user id
        let user_id = self.id;
        let _ = diesel::insert_into(users::table)
            .values(self)
            .execute(conn)?;
        Ok(user_id)
    }
}

// impl password hashing function
fn hash_password(pass: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(pass.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

#[derive(Debug, serde::Deserialize, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::Roles"]
pub enum Role {
    Client,
    Consultant,
    Builder,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::Client => "client",
            Role::Consultant => "consultant",
            Role::Builder => "builder",
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Client => write!(f, "Client"),
            Role::Consultant => write!(f, "Consultant"),
            Role::Builder => write!(f, "Builder"),
        }
    }
}

impl From<Role> for String {
    fn from(role: Role) -> String {
        role.as_str().to_string()
    }
}

impl From<String> for Role {
    fn from(role: String) -> Role {
        match role.as_str() {
            "client" => Role::Client,
            "consultant" => Role::Consultant,
            "builder" => Role::Builder,
            _ => panic!("Unknown role: {}", role),
        }
    }
}
