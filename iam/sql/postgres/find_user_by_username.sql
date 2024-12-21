SELECT t.uuid as tenant_id,
       u.username AS username,
       u.password AS password,
       u.enabled AS enabled,
       u.start_date AS start_date,
       u.end_date AS end_date,
       p.first_name AS first_name,
       p.last_name AS last_name,
       p.email_address AS email_address,
       p.street_name AS street_name,
       p.building_number AS building_number,
       p.postal_code AS postal_code,
       p.city AS city,
       p.state_province AS state_province,
       p.country_code AS country_code,
       p.primary_telephone AS primary_telephone,
       p.secondary_telephone AS secondary_telephone
  FROM "user" u
       INNER JOIN person p ON p.id = u.id
       INNER JOIN tenant t ON t.id = u.tenant_id
 WHERE t.uuid = $1
   AND u.username = $2