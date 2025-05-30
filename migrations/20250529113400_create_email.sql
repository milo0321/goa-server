CREATE TABLE email_accounts (
                               id SERIAL PRIMARY KEY,
                               email_address TEXT,
                               imap_server TEXT,
                               imap_port INTEGER,
                               username TEXT,
                               password TEXT,
                               use_ssl BOOLEAN,
                               created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE emails (
                        id SERIAL PRIMARY KEY,
                        config_id INTEGER REFERENCES email_configs(id),
                        subject TEXT,
                        sender TEXT,
                        received_at TIMESTAMP,
                        body TEXT
);

CREATE TABLE email_attachments (
                                   id SERIAL PRIMARY KEY,
                                   email_id INTEGER REFERENCES emails(id),
                                   filename TEXT,
                                   filepath TEXT,
                                   mimetype TEXT
);
