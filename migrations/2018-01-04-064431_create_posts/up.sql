CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  email TEXT NOT NULL,
  username TEXT NOT NULL,
  password TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (email, username)
);

 INSERT INTO users (id, email, username, password, created_at) VALUES
  (1, 'admin@163.com', 'admin', '$2y$12$yXTjrGePVLBPUH6YVs2f5OsUEGSotZxdL5Uu/70r63I5GtynVVjkK', '2017-09-08 13:00:26.353041'),
  (2, 'aaaa@163.com', 'aaaa', '$2y$12$3lOwd/qun2g.KBQpYz7DQu4HgreLODO4aJgYwFAQNj2AqgS14DAMK', '2017-09-08 13:00:28.353041'),
  (3, 'zzzz@163.com', 'zzzz', '$2y$12$6ofSZ3hpsGtDt6bM0WU0geDgZLLETFUVB6FpMXI61SbAvuQD5RiWK', '2017-09-08 13:00:38.353041');
 SELECT setval('users_id_seq', 3, true);

CREATE TABLE  themes (
  id SERIAL NOT NULL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  category_id INTEGER NOT NULL,
  theme_status INTEGER NOT NULL DEFAULT '0',
  title TEXT NOT NULL,
  content TEXT NOT NULL,
  view_count INTEGER NOT NULL DEFAULT '0',
  comment_count INTEGER NOT NULL DEFAULT '0',
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

 INSERT INTO themes (id, user_id, category_id, theme_status, title, content, view_count, comment_count, created_at) VALUES
 (1, 1, 1,  0, '1Rust Article', 'Rust 2017 Survey Results', 0, 3, '2017-07-24 23:41:45.353041'),
 (2, 2, 2, 0, '2The Rust Libz Blitz','This post covers the library team’s major initiative: raising a solid core of the Rust crate ecosystem to a consistent level of completeness and quality. ', 0, 3, '2017-07-23 23:41:45.353041'),
 (3, 3, 3, 0, '3Rust 2017 roadmap', 'This year, the overarching theme is productivity, especially for early-stage Rust users. ', 0, 1, '2017-07-23 23:41:45.353041'),
 (4, 1, 4, 0, '4Incremental Compilation', 'One of the projects that is building on these foundations, and that should help improve compile times a lot for typical workflows, is incremental compilation. ', 0, 1, '2017-07-24 23:41:45.353041'),
 (5, 2, 5, 0, '5Rust jobs','Today we are announcing an alpha version of incremental compilation', 0, 1, '2017-07-23 23:41:45.353041'),
 (6, 3, 2, 0, '6Introducing MIR','MIR is the key to ticking off a number of our highest priorities for Rust', 0, 0, '2017-07-23 23:41:45.353041'),
 (7, 1, 5,  0, '7MIR Compilation', 'overarching 2017 Survey Results', 0, 0, '2017-07-24 23:41:45.353041'),
 (8, 2, 3,  0, '8Results productivity', 'announcing 2017 Survey Results', 0, 0, '2017-07-24 23:41:45.353041'),
 (9, 1, 4,  0, '9One Survey', 'overarching 2017 Survey Results', 0, 0, '2017-07-24 23:41:45.353041'),
 (10, 2, 2,  0, '10Blitz productivity', 'announcing 2017 Survey Results', 0, 0, '2017-07-24 23:41:45.353041'),
 (11, 1, 3,  0, '11Survey ticking', 'overarching 2017 Survey Results', 0, 0, '2017-07-24 23:41:45.353041'),
 (12, 1, 4,  0, '12Rust Article', 'Rust 2017 Survey Results', 0, 3, '2017-07-24 23:41:45.353041'),
 (13, 2, 5, 0, '13The Rust Libz Blitz','This post covers the library team’s major initiative: raising a solid core of the Rust crate ecosystem to a consistent level of completeness and quality. ', 0, 3, '2017-07-23 23:41:45.353041'),
 (14, 3, 2, 0, '14Rust 2017 roadmap', 'This year, the overarching theme is productivity, especially for early-stage Rust users. ', 0, 1, '2017-07-23 23:41:45.353041'),
 (15, 1, 3, 0, '15Incremental Compilation', 'One of the projects that is building on these foundations, and that should help improve compile times a lot for typical workflows, is incremental compilation. ', 0, 1, '2017-07-24 23:41:45.353041'),
 (16, 2, 4, 0, '16Rust jobs','Today we are announcing an alpha version of incremental compilation', 0, 1, '2017-07-23 23:41:45.353041'),
 (17, 3, 5, 0, '17Introducing MIR','MIR is the key to ticking off a number of our highest priorities for Rust', 0, 0, '2017-07-23 23:41:45.353041'),
 (18, 1, 2,  0, '18MIR Compilation', 'overarching 2017 Survey Results', 0, 0, '2017-07-24 23:41:45.353041'),
 (19, 2, 3,  0, '19Results productivity', 'announcing 2017 Survey Results', 0, 0, '2017-07-24 23:41:45.353041'),
 (20, 1, 4,  0, '20One Survey', 'overarching 2017 Survey Results', 0, 0, '2017-07-24 23:41:45.353041'),
 (21, 2, 5,  0, '21Blitz productivity', 'announcing 2017 Survey Results', 0, 0, '2017-07-24 23:41:45.353041'),
 (22, 1, 2,  0, '22Survey ticking', 'overarching 2017 Survey Results', 0, 0, '2017-07-24 23:41:45.353041');
 SELECT setval('themes_id_seq', 22, true);

CREATE TABLE  categorys (
  id SERIAL NOT NULL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  category_name TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

 INSERT INTO categorys (id, user_id, category_name, created_at) VALUES
  (1, 1, 'office', '2017-09-08 13:00:26.353041'),
  (2, 2, 'blog', '2017-09-08 13:00:28.353041'),
  (3, 3, 'faq', '2017-09-08 13:00:38.353041'),
  (4, 1, 'share', '2017-09-08 13:00:26.353041'),
  (5, 2, 'job', '2017-09-08 13:00:28.353041');
 SELECT setval('categorys_id_seq', 5, true);


 CREATE TABLE  comments (
  id SERIAL NOT NULL PRIMARY KEY,
  theme_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  content TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

 INSERT INTO comments (id, theme_id, user_id, content, created_at) VALUES
 (1, 1, 1, 'Faster execution time', '2017-07-23 23:41:45.353041'),
 (2, 1, 1, 'Faster compilation time', '2017-07-23 23:41:45.353041'),
 (3, 3, 2, 'More precise type checking.', '2017-07-23 23:41:45.353041'),
 (4, 2, 2, 'Eliminating redundancy！', '2017-07-23 23:41:45.353041'),
 (5, 4, 2, 'Raising ambitions！', '2017-07-23 23:41:45.353041'),
 (6, 5, 2, 'MIR construction is type-driven', '2017-07-23 23:41:45.353041'),
 (7, 2, 2, 'Some MIR primitives are more powerful than the structured construct they replace', '2017-07-23 23:41:45.353041'),
 (8, 2, 2, 'MIR makes all types explicit', '2017-07-23 23:41:45.353041'),
 (9, 1, 1, 'Faster execution time', '2017-07-23 23:41:45.353041');
  SELECT setval('comments_id_seq', 9, true);

  CREATE TABLE messages (
  id SERIAL NOT NULL PRIMARY KEY,
  theme_id INTEGER NOT NULL,
  comment_id INTEGER NOT NULL,
  from_user_id INTEGER NOT NULL,
  to_user_id INTEGER NOT NULL,
  content TEXT NOT NULL,
  types INTEGER NOT NULL DEFAULT '0',
  message_status INTEGER NOT NULL DEFAULT '0',
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

  CREATE TABLE saves (
  id SERIAL NOT NULL PRIMARY KEY,
  theme_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

 INSERT INTO saves (id, theme_id, user_id, created_at) VALUES
 (1, 9, 1, '2017-07-23 23:41:45.353041'),
 (2, 6, 1, '2017-07-23 23:41:45.353041'),
 (3, 3, 2, '2017-07-23 23:41:45.353041'),
 (4, 2, 3, '2017-07-23 23:41:45.353041'),
 (5, 4, 2, '2017-07-23 23:41:45.353041'),
 (6, 5, 2, '2017-07-23 23:41:45.353041'),
 (7, 2, 2, '2017-07-23 23:41:45.353041'),
 (8, 2, 2, '2017-07-23 23:41:45.353041'),
 (9, 1, 1, '2017-07-23 23:41:45.353041');
  SELECT setval('saves_id_seq', 9, true);

