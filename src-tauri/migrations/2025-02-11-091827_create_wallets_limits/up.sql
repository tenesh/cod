-- Your SQL goes here
CREATE TABLE wallets_limits
(
	wallet_id INTEGER NOT NULL,
	limit_id    INTEGER NOT NULL,
	created_at     TEXT    NOT NULL DEFAULT current_timestamp,
	updated_at     TEXT    NOT NULL DEFAULT current_timestamp,
	PRIMARY KEY (wallet_id, limit_id),
	FOREIGN KEY (wallet_id) REFERENCES wallets (id)
		ON DELETE CASCADE
		ON UPDATE CASCADE,
	FOREIGN KEY (limit_id) REFERENCES limits (id)
		ON DELETE CASCADE
		ON UPDATE CASCADE
);

CREATE TRIGGER update_wallets_limits_updated_at
	AFTER UPDATE
	ON wallets_limits
	FOR EACH ROW
	WHEN old.updated_at <> current_timestamp
BEGIN
	UPDATE wallets_limits
	SET updated_at = current_timestamp
	WHERE wallet_id = old.wallet_id
	  AND limit_id = old.limit_id;
END;