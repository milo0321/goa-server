CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE orders
(
    id                    UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    order_number          TEXT        NOT NULL,
    customer_id           UUID        NOT NULL REFERENCES customers (id) ON DELETE CASCADE,
    customer_order_number TEXT        NOT NULL,
    quotation_id          UUID REFERENCES quotations (id),
    article               TEXT        NOT NULL,
    quantity              INTEGER     NOT NULL,
    unit_price            decimal     NOT NULL,
    currency              TEXT        NOT NULL DEFAULT 'USD',
    costs                 JSONB,
    packing_details       JSONB,
    notes                 TEXT,
    status                TEXT        NOT NULL CHECK (status IN ('draft', 'quoted', 'ordered', 'canceled')),
    order_date            TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_orders_customer_id ON orders (customer_id);
CREATE INDEX idx_orders_status ON orders (status);
CREATE INDEX idx_orders_article ON orders (article);
CREATE INDEX idx_orders_order_date ON orders (order_date);