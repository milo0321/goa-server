CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE quotations (
                            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                            customer_id UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
                            article TEXT NOT NULL,
                            client TEXT,
                            size TEXT,
                            material TEXT,
                            color TEXT,
                            details TEXT,
                            branding TEXT,
                            packing TEXT,
                            quantity TEXT,
                            certifications TEXT,
                            quantity_tiers JSONB,
                            additional_fees JSONB,
                            notes TEXT,
                            status TEXT NOT NULL CHECK (status IN ('draft', 'quoted', 'ordered', 'canceled')),
                            inquiry_date TIMESTAMPTZ NOT NULL DEFAULT now(),
                            created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                            updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);



CREATE INDEX idx_quotations_customer_id ON quotations(customer_id);
CREATE INDEX idx_quotations_status ON quotations(status);
CREATE INDEX idx_quotations_article ON quotations(article);
CREATE INDEX idx_quotations_inquiry_date ON quotations(inquiry_date);

ALTER TABLE quotations
    ALTER COLUMN quantity_tiers TYPE JSONB
    USING quantity_tiers::JSONB;