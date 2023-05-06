# Axum-Rust-Rest-Api-Template

This project is an open source Rest Api Template built with Rust's Axum framework.

## Why this project?

I have been learning rust for some time now and I noticed a lack of open source rust Rest ApI template template that define directory structure the way I like and I'm used to ( SOA ). Most other projects have all the code placed in a single `main.rs` file.
## Features

This project uses Axum framework and SQLx for DB access layer, as well as other wonderful packages. It includes three basic routes: register, login, and profile page.

## Getting Started

1. Clone the project
2. Update `.env` file with your DB credentials
3. Install `sqlx-cli` or run `cargo sqlx database create` to create your DB
4. Run the migration file using `cargo sqlx migrate run`. This will run the migration file that exists in the migration folder in the root of the project.
5. Build the project and dependencies using `cargo build`
6. Run the project using `cargo run -- up`

For this project, I used MySQL has my DB driver. You can use any other DB you like, but you will need to update the `.env` file with the appropriate DB connection url, Also you will need to update `src/config/database.rs` so sqlx can use your new driver.
For example, if you want to use postgres, you will need to update the `src/config/database.rs` file to look like this:

```
use crate::parameter;
use async_trait::async_trait;
use sqlx::{Error, Pool, Postgres};

pub struct Database {
    pool: Pool<Postgres>,
}

#[async_trait]
pub trait DatabaseTrait {
    async fn init() -> Result<Self, Error>
        where
            Self: Sized;
    fn get_pool(&self) -> &Pool<Postgres>;
}

#[async_trait]
impl DatabaseTrait for Database {
    async fn init() -> Result<Self, Error> {
        let database_url = parameter::get("DATABASE_URL");
        let pool = Postgres::connect(&database_url).await?;
        Ok(Self { pool })
    }

    fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}

You will also need to update the migration files in `migrations/*` to match.
```

## Contributing

Contributions are always welcome! 
