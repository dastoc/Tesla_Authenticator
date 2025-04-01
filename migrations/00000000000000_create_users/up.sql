CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL UNIQUE,
    password_hash VARCHAR NOT NULL
);

-- Insert a test user if not already present
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM users WHERE username = 'testuser') THEN
        INSERT INTO users (username, email, password_hash)
        VALUES (
            'testuser',
            'test@example.com',
            '$2b$12$K8xX8x8x8x8x8x8x8x8x8e' -- Pre-hashed password "test123"
        );
    END IF;
END $$;