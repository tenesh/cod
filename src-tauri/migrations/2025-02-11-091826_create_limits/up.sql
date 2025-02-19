-- Your SQL goes here
CREATE TABLE limits
(
	id                   INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	name                 TEXT    NOT NULL,
	description          TEXT    NULL,
	interval             TEXT    NOT NULL CHECK (interval IN
												 ('Hourly', 'Daily', 'Weekly', 'Biweekly', 'Monthly', 'Quarterly',
												  'Semiannually', 'Annually', 'Biennially', 'Custom')),
	custom_interval_days INTEGER CHECK ((interval = 'Custom' AND custom_interval_days > 0) OR
										(interval <> 'Custom' AND custom_interval_days IS NULL)),
	amount               REAL    NOT NULL DEFAULT 0 CHECK (amount >= 0),
	created_at           TEXT    NOT NULL DEFAULT current_timestamp,
	updated_at           TEXT    NOT NULL DEFAULT current_timestamp
);

CREATE TRIGGER update_limits_updated_at
	AFTER UPDATE
	ON limits
	FOR EACH ROW
	WHEN old.updated_at <> current_timestamp
BEGIN
	UPDATE limits
	SET updated_at = current_timestamp
	WHERE id = old.id;
END;
