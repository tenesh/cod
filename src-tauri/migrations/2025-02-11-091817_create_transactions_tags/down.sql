-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS update_transactions_tags_updated_at;
DROP TABLE IF EXISTS transactions_tags;