-- 创建报价表
CREATE TABLE quotations (
                            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                            inquiry_date TIMESTAMPTZ NOT NULL,
                            customer_id UUID NOT NULL REFERENCES customers(id) ON DELETE CASCADE,
                            product_name TEXT NOT NULL,
                            quantity INTEGER NOT NULL CHECK (quantity > 0),
                            status TEXT NOT NULL CHECK (status IN ('pending', 'quoted')),
                            quoted_price NUMERIC(12, 2),
                            quoted_date TIMESTAMPTZ,
                            notes TEXT,
                            created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
                            updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 创建索引
CREATE INDEX idx_quotations_customer_id ON quotations(customer_id);
CREATE INDEX idx_quotations_status ON quotations(status);