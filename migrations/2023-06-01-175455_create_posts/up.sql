-- Your SQL goes here

CREATE TABLE posts (
  id INT AUTO_INCREMENT PRIMARY KEY,
  title NVARCHAR(50) NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL
);
