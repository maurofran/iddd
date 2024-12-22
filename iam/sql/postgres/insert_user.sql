WITH iu AS (
    INSERT INTO "user" (tenant_id, username, password, enabled, start_date, end_date, created_at, updated_at)
    SELECT t.id, $2, $3, $4, $5, $6, current_timestamp, current_timestamp
      FROM tenant t
     WHERE t.uuid = $1
    RETURNING id AS user_id
)
INSERT INTO person (id, first_name, last_name, email_address, street_name, building_number, postal_code, city, state_province, country_code, primary_telephone, secondary_telephone)
SELECT user_id, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17
  FROM iu