CREATE TABLE IF NOT EXISTS pictures
(
  id          VARCHAR(36) PRIMARY KEY NOT NULL,
  flickr_id   VARCHAR(36) UNIQUE NOT NULL,
  monument_id VARCHAR(36) NOT NULL,
  license_id  VARCHAR(36) NOT NULL,
  url         TEXT NOT NULL,
  author      TEXT NOT NULL,

  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

ALTER TABLE pictures
      ADD FOREIGN KEY (monument_id) REFERENCES monuments (id),
      ADD FOREIGN KEY (license_id) REFERENCES licenses (id);
