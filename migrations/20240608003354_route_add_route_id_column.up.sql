-- Add up migration script here
INSERT INTO route_status(code, description)
VALUES ('CREATED', 'Rota criada')
ON CONFLICT (code)
    DO NOTHING;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM information_schema.columns
        WHERE table_name = 'routes'
          AND column_name = 'status_id'
    ) THEN
        ALTER TABLE IF EXISTS routes
            ADD COLUMN status_id UUID
                CONSTRAINT fk_routes_route_status REFERENCES route_status (id);
    END IF;
END $$;

UPDATE routes
SET status_id = (SELECT id FROM route_status WHERE code = 'CREATED')
WHERE status_id IS NULL;

ALTER TABLE IF EXISTS routes ALTER COLUMN status_id SET NOT NULL;
