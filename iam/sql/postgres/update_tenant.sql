UPDATE tenant
   SET name = $2,
       description = $3,
       enabled = $4,
       updated_at = current_timestamp
 WHERE uuid = $1