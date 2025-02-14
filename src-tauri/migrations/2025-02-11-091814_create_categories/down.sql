-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS update_categories_updated_at;
DROP TABLE IF EXISTS categories;