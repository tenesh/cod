-- Your SQL goes here
CREATE TABLE transactions_categories
(
	transaction_id INTEGER NOT NULL,
	category_id    INTEGER NOT NULL,
	created_at     TEXT    NOT NULL DEFAULT current_timestamp,
	updated_at     TEXT    NOT NULL DEFAULT current_timestamp,
	PRIMARY KEY (transaction_id, category_id),
	FOREIGN KEY (transaction_id) REFERENCES transactions (id)
		ON DELETE CASCADE
		ON UPDATE CASCADE,
	FOREIGN KEY (category_id) REFERENCES categories (id)
		ON DELETE CASCADE
		ON UPDATE CASCADE
);

CREATE TRIGGER update_transactions_categories_updated_at
	AFTER UPDATE
	ON transactions_categories
	FOR EACH ROW
	WHEN old.updated_at <> current_timestamp
BEGIN
	UPDATE transactions_categories
	SET updated_at = current_timestamp
	WHERE transaction_id = old.transaction_id
	  AND category_id = old.category_id;
END;