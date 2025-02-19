-- Your SQL goes here
CREATE TABLE accounts_limits
(
	account_id INTEGER NOT NULL,
	limit_id    INTEGER NOT NULL,
	created_at     TEXT    NOT NULL DEFAULT current_timestamp,
	updated_at     TEXT    NOT NULL DEFAULT current_timestamp,
	PRIMARY KEY (account_id, limit_id),
	FOREIGN KEY (account_id) REFERENCES accounts (id)
		ON DELETE CASCADE
		ON UPDATE CASCADE,
	FOREIGN KEY (limit_id) REFERENCES limits (id)
		ON DELETE CASCADE
		ON UPDATE CASCADE
);

CREATE TRIGGER update_accounts_limits_updated_at
	AFTER UPDATE
	ON accounts_limits
	FOR EACH ROW
	WHEN old.updated_at <> current_timestamp
BEGIN
	UPDATE accounts_limits
	SET updated_at = current_timestamp
	WHERE account_id = old.account_id
	  AND limit_id = old.limit_id;
END;