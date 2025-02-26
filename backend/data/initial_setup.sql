INSERT INTO
  roles (name)
VALUES
  ('Admin'),
  ('User')
ON CONFLICT DO NOTHING;

INSERT INTO
  users (name, email, password_hash, role_id)
SELECT
  'Eleazar Fig',
  'eleazar.fig@example.com',
  '$2b$12$DTlCrx8wevNy9/SAh/bPauS937AO4n7OYraUrROdyHIz7Hnaja91C',
  role_id
FROM
  roles
WHERE
  name LIKE 'Admin';