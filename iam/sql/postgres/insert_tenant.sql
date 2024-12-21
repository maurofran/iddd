INSERT INTO tenant (uuid, name, description, enabled, created_at, updated_at)
VALUES ($1, $2, $3, $4, current_timestamp, current_timestamp)