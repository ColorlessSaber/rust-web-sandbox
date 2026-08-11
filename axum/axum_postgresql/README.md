make sure to install the sqlx-cli
```bash
cargo install sqlx-cli --no-default-features --features rustls,postgres
```

You will also need to spin-up a PostgreSQL container in Docker or Podman. Below is how to create and run a
container in Podman:
```bash
sudo podman run -d \
> --name postgresql-database \
> -p 5432:5432 \
> -e POSTGRES_PASSWORD=postgres \
> docker.io/library/postgres:16
```

Use the following SQL command to create the book_db in PostgreSQL.
```postgresql
CREATE TABLE book_db (
    id SERIAL primary KEY,
    title VARCHAR(255),
    author VARCHAR(255),
    genre VARCHAR(50)
);
```