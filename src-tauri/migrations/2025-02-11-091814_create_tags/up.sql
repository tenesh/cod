-- Your SQL goes here
CREATE TABLE tags
(
	id          INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	name        TEXT    NOT NULL,
	description TEXT    NULL,
	color       TEXT    NOT NULL,
	user_id     INTEGER NOT NULL,
	created_at  TEXT    NOT NULL DEFAULT current_timestamp,
	updated_at  TEXT    NOT NULL DEFAULT current_timestamp,
	FOREIGN KEY (user_id) REFERENCES users (id)
		ON DELETE CASCADE
		ON UPDATE CASCADE
);

CREATE TRIGGER update_tags_updated_at
	AFTER UPDATE
	ON tags
	FOR EACH ROW
	WHEN old.updated_at <> current_timestamp
BEGIN
	UPDATE tags
	SET updated_at = current_timestamp
	WHERE id = old.id;
END;