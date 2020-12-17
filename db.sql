CREATE TABLE users
(
  id uuid NOT NULL,
  email varchar(100) NOT NULL,
  password varchar(150) NOT NULL,
  created_at timestamp WITHOUT TIME ZONE DEFAULT (NOW() AT TIME ZONE 'UTC') NOT NULL,
  updated_at timestamp WITHOUT TIME ZONE NULL,
  role varchar(20) NOT NULL,
  PRIMARY KEY (id),
  UNIQUE (email)
);

INSERT INTO "users" ("id", "email", "password", "created_at", "updated_at", "role") VALUES
('a1873aaa-266c-4ae9-b731-400cce5885a3',	'admin@test.com',	'$argon2id$v=19$m=4096,t=192,p=8$SPKprG7NQ1i2mWfwMhuOUfloisnPN4CDzybDrctF6UQ$Va5THpwo7ghMofCRQA+ztraYM3h091g5psUWVtCUsGI',	'2020-12-14 02:31:59.653056',	NULL,	'Admin'),
('551354df-1929-4a72-b965-6cc14a03d6ee',	'user@test.com',	'$argon2id$v=19$m=4096,t=192,p=8$jU6csuL4zLbWG5mgcZ0q5KhL1ynRvsk1/OmG4QGIFvo$6T+E9AY/YPyxxpwW4oE2biZF57PNMIrWAyyop+RMHcg',	'2020-12-14 02:33:34.991789',	NULL,	'User');
