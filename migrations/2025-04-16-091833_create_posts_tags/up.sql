-- Your SQL goes here
CREATE TABLE posts_tags (
    post_id INT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    tag VARCHAR NOT NULL,
    PRIMARY KEY (post_id, tag)
);