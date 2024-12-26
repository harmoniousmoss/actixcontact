# ActixContact

This repository contains the backend implementation for handling contact form submissions from the website. It is built using **Actix Web**, a fast and lightweight web framework for Rust, and **PostgreSQL** as the database to store and manage contact form data.

## Features

- **Contact Form Handling**: Receive and process form submissions with validation.
- **Data Persistence**: Store form data securely in a PostgreSQL database.
- **API Endpoints**:
  - Submit a contact form.
  - Retrieve contact submissions (admin access).
- **Error Handling**: Comprehensive error responses for invalid inputs or system issues.

## Tech Stack

- **Rust**: Programming language for backend development.
- **Actix Web**: Web framework for building efficient APIs.
- **PostgreSQL**: Relational database for data storage.
- **SQLx**: Async, compile-time verified ORM for database interaction.
- **dotenv**: For environment variable management.
- **bcrypt**: For secure password hashing.
- **jsonwebtoken**: For JWT-based authentication.

## Development Plan
- [x] Seed database with admin initial data.
- [x] Implement JWT-based authentication.
- [x] Add API endpoint for retrieving contact submissions.
