CREATE TABLE IF NOT EXISTS licenses
(
  id         VARCHAR(36) PRIMARY KEY NOT NULL,
  flickr_id  INT UNIQUE NOT NULL,
  name       TEXT NOT NULL,
  url        TEXT DEFAULT NULL,

  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
