# Axum-Rust-Api-Boilerplate

This project is an open source boilerplate built with Rust's Axum framework.

## Why this project?

I have been learning rust for some time now and I noticed a lack of open source axum boilerplates that define directory structure the way I like and I'm used to. Most other projects have all the code placed in a single `main.rs` file.
## Features

This project uses Axum framework and SQLx for DB access layer, as well as other wonderful packages. It includes three basic routes: register, login, and profile page.

## Getting Started

1. Clone the project
2. Update `.env` file with your DB credentials
3. Install `sqlx-cli` or run `cargo sqlx database create` to create your DB
4. Run the migration file using `cargo sqlx migrate run`. This will run the migration file that exists in the migration folder in the root of the project.
5. Build the project and dependencies using `cargo build`
6. Run the project using `cargo run -- up`

## Contributing

Contributions are always welcome! 
