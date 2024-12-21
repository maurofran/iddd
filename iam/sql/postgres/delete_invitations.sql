DELETE FROM invitation i
 USING tenant t
 WHERE i.tenant_id = t.id
   AND t.uuid = $1
   AND i.identifier <> ANY($2)