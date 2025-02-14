-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS update_wallets_limits_updated_at;
DROP TABLE IF EXISTS wallets_limits;