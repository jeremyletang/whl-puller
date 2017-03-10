CREATE TABLE IF NOT EXISTS last_updates
(
  id                      VARCHAR(36) PRIMARY KEY NOT NULL,
  monument_id             VARCHAR(36) NOT NULL,

  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

ALTER TABLE last_updates ADD FOREIGN KEY (monument_id) REFERENCES monuments (id);
