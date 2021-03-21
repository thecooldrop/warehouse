# Introduction to web development with Rust

## Running a local database in the container
To run local database with negligble overhead we can provide a local container to run the database for our 
development efforts. The Postgres database is very good for this. Please refer to Postgres DockerHub page for guidance
on how to start a local database. 

## Configuring Diesel

To configure Diesel we need to provide a database url via `DATABASE_URL` environment variable 