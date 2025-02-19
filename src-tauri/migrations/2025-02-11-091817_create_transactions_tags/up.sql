-- Your SQL goes here
CREATE TABLE transactions_tags
(
	transaction_id INTEGER NOT NULL,
	tag_id         INTEGER NOT NULL,
	created_at     TEXT    NOT NULL DEFAULT current_timestamp,
	updated_at     TEXT    NOT NULL DEFAULT current_timestamp,
	PRIMARY KEY (transaction_id, tag_id),
	FOREIGN KEY (transaction_id) REFERENCES transactions (id)
		ON DELETE CASCADE
		ON UPDATE CASCADE,
	FOREIGN KEY (tag_id) REFERENCES tags (id)
		ON DELETE CASCADE
		ON UPDATE CASCADE
);

CREATE TRIGGER update_transactions_tags_updated_at
	AFTER UPDATE
	ON transactions_tags
	FOR EACH ROW
	WHEN old.updated_at <> current_timestamp
BEGIN
	UPDATE transactions_tags
	SET updated_at = current_timestamp
	WHERE transaction_id = old.transaction_id
	  AND tag_id = old.tag_id;
END;