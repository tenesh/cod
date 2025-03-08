-- Your SQL goes here
CREATE TABLE currencies
(
	id             INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	name           TEXT    NOT NULL,
	code           TEXT    NOT NULL UNIQUE,
	symbol         TEXT    NOT NULL,
	decimal_digits REAL    NOT NULL CHECK (decimal_digits >= 0),
	rounding       REAL    NOT NULL CHECK (rounding >= 0),
	created_at     TEXT    NOT NULL DEFAULT current_timestamp,
	updated_at     TEXT    NOT NULL DEFAULT current_timestamp
);

CREATE TRIGGER update_currencies_updated_at
	AFTER UPDATE
	ON currencies
	FOR EACH ROW
	WHEN old.updated_at <> current_timestamp
BEGIN
	UPDATE currencies
	SET updated_at = current_timestamp
	WHERE id = old.id;
END;