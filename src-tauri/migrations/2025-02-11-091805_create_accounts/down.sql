-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS update_accounts_updated_at;
DROP TABLE IF EXISTS accounts;