CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE order_status AS ENUM (
    'draft',
    'sampling',
    'sample_approved',
    'mass_production',
    'ready_to_ship',
    'shipped',
    'completed',
    'cancelled'
    );

CREATE TABLE orders
(
    id                UUID PRIMARY KEY      DEFAULT gen_random_uuid(),
    order_no          TEXT         NOT NULL,
    order_article     TEXT         NOT NULL,
    customer_id       UUID         NOT NULL REFERENCES customers (id) ON DELETE CASCADE,
    customer_order_no TEXT         NOT NULL,
    quotation_id      UUID REFERENCES quotations (id),
    article           TEXT         NOT NULL,
    currency          TEXT         NOT NULL DEFAULT 'USD',
    payment_terms     TEXT         NOT NULL,
    delivery_time     TIMESTAMPTZ  NOT NULL DEFAULT now(),
    shipping_method   TEXT         NOT NULL,
    remarks           TEXT,
    status            order_status NOT NULL DEFAULT 'draft',
    packing_details   JSONB,
    order_date        TIMESTAMPTZ  NOT NULL DEFAULT now(),
    created_at        TIMESTAMPTZ  NOT NULL DEFAULT now(),
    updated_at        TIMESTAMPTZ  NOT NULL DEFAULT now()
);

CREATE INDEX idx_orders_customer_id ON orders (customer_id);
CREATE INDEX idx_orders_status ON orders (status);
CREATE INDEX idx_orders_article ON orders (article);
CREATE INDEX idx_orders_order_date ON orders (order_date);