SELECT t.uuid AS tenant_id,
       t.name AS tenant_name,
       t.description AS tenant_description,
       t.enabled AS tenant_enabled,
       i.identifier AS invitation_id,
       i.description AS invitation_description,
       i.valid_from,
       i.until
  FROM tenant t
       INNER JOIN invitation i ON i.tenant_id = i.id
 WHERE t.name = $1