CREATE TABLE IF NOT EXISTS servers
(
    id         BIGSERIAL   PRIMARY KEY,
    discord_id BIGINT      NOT NULL,
    prefix     VARCHAR(10)
);
