CREATE TABLE IF NOT EXISTS pictures
(
  id          VARCHAR(36) PRIMARY KEY NOT NULL,
  flickr_id   INT UNIQUE,
  monument_id VARCHAR(36),
  license_id  VARCHAR(36),
  url         TEXT,
  author      TEXT,

  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

ALTER TABLE pictures
      ADD FOREIGN KEY (monument_id) REFERENCES monuments (id),
      ADD FOREIGN KEY (license_id) REFERENCES licenses (id);
