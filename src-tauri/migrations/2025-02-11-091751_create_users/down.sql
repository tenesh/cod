-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS update_users_updated_at;
DROP TABLE IF EXISTS users;