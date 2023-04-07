-- Your SQL goes here
CREATE TABLE weights(
    id INT AUTO_INCREMENT PRIMARY KEY,
    timestamp DATETIME NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    weight FLOAT NOT NULL,
    index timestamp_index(timestamp)
);