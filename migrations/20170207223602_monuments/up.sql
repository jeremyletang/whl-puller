CREATE TABLE IF NOT EXISTS monuments
(
  id                      VARCHAR(36) PRIMARY KEY NOT NULL,
  category                TEXT        DEFAULT NULL,
  criteria_txt            TEXT        DEFAULT NULL,
  danger                  TEXT        DEFAULT NULL,
  date_inscribed          TEXT        DEFAULT NULL,
  extension               INT         DEFAULT NULL,
  historical_description  TEXT        DEFAULT NULL,
  http_url                TEXT        DEFAULT NULL,
  id_number               INT         DEFAULT NULL,
  image_url               TEXT        DEFAULT NULL,
  iso_code                TEXT        DEFAULT NULL,
  justification           TEXT        DEFAULT NULL,
  latitude                REAL        DEFAULT NULL,
  longitude               REAL        DEFAULT NULL,
  location                TEXT        DEFAULT NULL,
  long_description        TEXT        DEFAULT NULL,
  region                  TEXT        DEFAULT NULL,
  revision                INT         DEFAULT NULL,
  secondary_dates         TEXT        DEFAULT NULL,
  short_description       TEXT        DEFAULT NULL,
  site                    TEXT        DEFAULT NULL,
  states                  TEXT        DEFAULT NULL,
  transboundary           INT         DEFAULT NULL,
  unique_number           INT         DEFAULT NULL,

  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE OR REPLACE FUNCTION update_modified_timestamp() RETURNS TRIGGER
LANGUAGE plpgsql
AS
$$
BEGIN
    IF (NEW != OLD) THEN
       NEW.updated_at = CURRENT_TIMESTAMP;
       RETURN NEW;
    END IF;
    RETURN OLD;
END;
$$;
