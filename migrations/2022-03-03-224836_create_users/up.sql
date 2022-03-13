CREATE TABLE IF NOT EXISTS "users" (
	"id"	VARCHAR NOT NULL,
	"email"	VARCHAR NOT NULL,
	"subscribed"	BOOLEAN NOT NULL,
	"first_name"	VARCHAR NOT NULL,
	"last_name"	VARCHAR NOT NULL,
	PRIMARY KEY("id")
);