-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS update_limits_updated_at;
DROP TABLE IF EXISTS limits;