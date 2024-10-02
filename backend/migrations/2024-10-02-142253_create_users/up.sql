CREATE TYPE user_status AS ENUM ('verified', 'un_verified', 'suspend');

CREATE TABLE
  users (
    "id" SERIAL PRIMARY KEY,
    "created_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "email" VARCHAR(254) NOT NULL UNIQUE,
    "password" CHAR(93) NOT NULL,
    "profile" CHAR(40) NOT NULL DEFAULT '00000000-0000-0000-0000-000000000000.png',
    "status" user_status NOT NULL DEFAULT 'un_verified',
    "is_admin" BOOLEAN NOT NULL DEFAULT FALSE
  );