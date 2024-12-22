WITH usr AS (
    UPDATE "user" u
        SET version = u.version + 1,
            password = $3,
            enabled = $4,
            start_date = $5,
            end_date = $6,
            updated_at = current_timestamp
        FROM tenant t
        WHERE u.tenant_id = t.id
            AND t.uuid = $1
            AND u.username = $2
        RETURNING u.id AS user_id
)
UPDATE person p
SET first_name = $7,
    last_name = $8,
    email_address = $9,
    street_name = $10,
    building_number = $11,
    postal_code = $12,
    city = $13,
    state_province = $14,
    country_code = $15,
    primary_telephone = $16,
    secondary_telephone = $17
FROM usr
WHERE p.id = usr.user_id