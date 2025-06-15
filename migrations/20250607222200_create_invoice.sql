-- invoices table
CREATE TABLE invoices
(
    id             UUID PRIMARY KEY,
    invoice_number TEXT UNIQUE    NOT NULL,
    customer_id    UUID           NOT NULL REFERENCES customers (id) ON DELETE CASCADE,
    total_amount   NUMERIC(12, 2) NOT NULL,
    currency       TEXT           NOT NULL,
    invoice_type   TEXT           NOT NULL  DEFAULT 'pay',
    status         TEXT           NOT NULL  DEFAULT 'unpaid',
    issue_date     TIMESTAMP WITH TIME ZONE DEFAULT now(),
    created_at     TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at     TIMESTAMP WITH TIME ZONE DEFAULT now()
);

-- order_invoice junction table
CREATE TABLE order_invoice
(
    order_id   UUID REFERENCES orders (id) ON DELETE CASCADE,
    invoice_id UUID REFERENCES invoices (id) ON DELETE CASCADE,
    amount     NUMERIC(12, 2) NOT NULL,
    PRIMARY KEY (order_id, invoice_id)
);