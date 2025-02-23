-- Your SQL goes here
CREATE TABLE transactions
(
	id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	party           TEXT    NOT NULL,
	description     TEXT    NULL,
	currency_id     INTEGER NOT NULL,
	conversion_rate REAL    NOT NULL DEFAULT 1 CHECK (conversion_rate > 0),
	amount          REAL    NOT NULL DEFAULT 0 CHECK ((category = 'inflow' AND amount >= 0) OR
													  (category = 'outflow' AND amount <= 0)),
	category        TEXT    NOT NULL CHECK (category IN ('inflow', 'outflow')),
	medium          TEXT    NOT NULL CHECK (medium IN
											('cash', 'bank transfer', 'credit card', 'debit card', 'digital payment',
											 'crypto', 'check', 'mobile wallet', 'other')),
	status          TEXT    NOT NULL CHECK (status IN ('completed', 'declined', 'pending')),
	account_id      INTEGER NOT NULL,
	transacted_at   TEXT    NOT NULL DEFAULT current_timestamp,
	created_at      TEXT    NOT NULL DEFAULT current_timestamp,
	updated_at      TEXT    NOT NULL DEFAULT current_timestamp,
	FOREIGN KEY (currency_id) REFERENCES currencies (id)
		ON UPDATE CASCADE,
	FOREIGN KEY (account_id) REFERENCES accounts (id)
		ON DELETE CASCADE
		ON UPDATE CASCADE
);

CREATE TRIGGER update_transactions_updated_at
	AFTER UPDATE
	ON transactions
	FOR EACH ROW
	WHEN old.updated_at <> current_timestamp
BEGIN
	UPDATE transactions
	SET updated_at = current_timestamp
	WHERE id = old.id;
END;