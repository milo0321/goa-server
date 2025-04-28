CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE quotations (
                            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                            inquiry_date TIMESTAMPTZ NOT NULL,
                            customer_id UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
                            product_name TEXT NOT NULL,
                            quantity_type TEXT NOT NULL CHECK (quantity_type IN ('single', 'multiple')),
                            quantity_tiers JSONB NOT NULL,
                            status TEXT NOT NULL CHECK (status IN ('draft', 'quoted', 'ordered', 'expired')),
                            notes TEXT,
                            additional_fees JSONB,
                            created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                            updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_quotations_customer_id ON quotations(customer_id);
CREATE INDEX idx_quotations_status ON quotations(status);
CREATE INDEX idx_quotations_product_name ON quotations(product_name);
CREATE INDEX idx_quotations_inquiry_date ON quotations(inquiry_date);