INSERT INTO invitation (tenant_id, identifier, description, valid_from, until)
SELECT id, $2, $3, $4, $5
  FROM tenant
 WHERE uuid = $1
ON CONFLICT (tenant_id, identifier) DO UPDATE
    SET description = EXCLUDED.description,
        valid_from = EXCLUDED.valid_from,
        until = EXCLUDED.until