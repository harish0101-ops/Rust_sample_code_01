:

🚀 Rust Blog API using Rocket, Diesel, and PostgreSQL
This is a simple Blog backend API built using:

Rust

Rocket (web framework)

Diesel ORM

PostgreSQL (database)

🏗 Project Structure
├── Cargo.toml
├── src/
│   ├── db.rs           # Database connection setup
│   ├── models.rs       # Structs for Users, Posts, Tags
│   ├── routes/
│   │    ├── users.rs   # User APIs
│   │    ├── posts.rs   # Post APIs
│   ├── schema.rs       # Diesel generated schema
├── migrations/         # Diesel migration files
└── README.md

