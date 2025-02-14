-- Your SQL goes here
CREATE TABLE users
(
	id         INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	username   TEXT    NOT NULL,
	email      TEXT    NOT NULL UNIQUE,
	password   TEXT    NOT NULL,
	created_at TEXT    NOT NULL DEFAULT current_timestamp,
	updated_at TEXT    NOT NULL DEFAULT current_timestamp
);

CREATE TRIGGER update_users_updated_at
	AFTER UPDATE
	ON users
	FOR EACH ROW
	WHEN old.updated_at <> current_timestamp
BEGIN
	UPDATE users
	SET updated_at = current_timestamp
	WHERE id = old.id;
END;