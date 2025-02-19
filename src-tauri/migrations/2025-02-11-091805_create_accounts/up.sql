-- Your SQL goes here
CREATE TABLE accounts
(
	id          INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	name        TEXT    NOT NULL,
	description TEXT    NULL,
	currency_id INTEGER NOT NULL,
	user_id     INTEGER NOT NULL,
	created_at  TEXT    NOT NULL DEFAULT current_timestamp,
	updated_at  TEXT    NOT NULL DEFAULT current_timestamp,
	FOREIGN KEY (currency_id) REFERENCES currencies (id)
		ON UPDATE CASCADE,
	FOREIGN KEY (user_id) REFERENCES users (id)
		ON DELETE CASCADE
		ON UPDATE CASCADE
);

CREATE TRIGGER update_accounts_updated_at
	AFTER UPDATE
	ON accounts
	FOR EACH ROW
	WHEN old.updated_at <> current_timestamp
BEGIN
	UPDATE accounts
	SET updated_at = current_timestamp
	WHERE id = old.id;
END;