-- 0001_create_customers_table.sql

CREATE TABLE customers (
                           id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                           name TEXT NOT NULL,
                           email TEXT UNIQUE NOT NULL,
                           phone TEXT,
                           company TEXT,
                           position TEXT,
                           address TEXT,
                           created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                           updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);