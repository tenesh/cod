-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS update_wallets_updated_at;
DROP TABLE IF EXISTS wallets;