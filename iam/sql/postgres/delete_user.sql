DELETE
FROM "user" u
    USING tenant t
WHERE u.tenant_id = t.id
  AND t.uuid = $1
  AND u.username = $2