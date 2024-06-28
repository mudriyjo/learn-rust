-- Add migration script here
CREATE TABLE messages (
  id SERIAL PRIMARY KEY, 
  message VARCHAR (255) UNIQUE NOT NULL
);

INSERT INTO messages (id, message) VALUES (1, 'Hello world!');
INSERT INTO messages (id, message) VALUES (2, 'Absolutly new message');
INSERT INTO messages (id, message) VALUES (3, 'Message from Doctor who?');