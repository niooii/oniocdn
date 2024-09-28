CREATE TABLE IF NOT EXISTS media (
    id SERIAL PRIMARY KEY,
    time_uploaded   BIGINT NOT NULL,
    time_expiring   BIGINT NOT NULL,
    file_path       TEXT NOT NULL,
    file_size       INT NOT NULL,
    file_hash       TEXT NOT NULL
);  