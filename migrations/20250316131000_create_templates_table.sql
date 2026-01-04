CREATE TABLE IF NOT EXISTS templates (
    unid        UUID                    PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    name        CHARACTER VARYING(255)  NOT NULL,
    created_at  TIMESTAMPTZ             NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ             NOT NULL DEFAULT now()
);


-- Insert data into templates table
INSERT INTO templates (name) VALUES
('Value #1'),
('Value #2'),
('Value #3'),
('Value #4');


