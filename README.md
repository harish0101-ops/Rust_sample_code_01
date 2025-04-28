# 🚀 Rust Blog API using Rocket, Diesel, and PostgreSQL

This is a simple Blog backend API built with a focus on speed, safety, and robust data management. It leverages the power of Rust, the elegance of the Rocket web framework, the efficiency of the Diesel ORM, and the reliability of PostgreSQL.

## 🛠️ Technologies Used

* **Rust:** A systems programming language known for its performance, safety, and concurrency.
* **Rocket:** A web framework for Rust with a focus on developer experience and type safety.
* **Diesel ORM:** A safe and extensible Object Relational Mapper (ORM) for Rust, providing compile-time checks and preventing runtime errors when interacting with databases.
* **PostgreSQL:** A powerful, open-source relational database system known for its reliability and advanced features.

## 🏗️ Project Structure

The project is organized to separate concerns and improve maintainability:

├── Cargo.toml             # Rust project configuration and dependencies
├── src/                   # Source code directory
│   ├── db.rs              # Database connection setup using Diesel
│   ├── models.rs          # Struct definitions for Users, Posts, and Tags, representing database tables
│   ├── routes/            # Contains the API endpoint handlers
│   │   ├── users.rs       # API endpoints related to user management (e.g., create, read)
│   │   ├── posts.rs       # API endpoints for managing blog posts (e.g., create, read, update, delete)
│   ├── schema.rs          # Diesel-generated schema definition based on your database migrations
├── migrations/            # Directory containing Diesel migration files for database schema management
└── README.md              # This README file
