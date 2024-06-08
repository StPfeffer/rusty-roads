BEGIN;

DO $$
BEGIN
    IF EXISTS (SELECT 1 
               FROM information_schema.columns 
               WHERE table_name = 'routes' AND column_name = 'status_id') THEN
        ALTER TABLE routes
        DROP COLUMN status_id;
    END IF;
END $$;

DO $$
BEGIN
    DELETE FROM route_status
    WHERE code = 'CREATED';
END $$;

COMMIT;
