CREATE TABLE IF NOT EXISTS projects (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  name        TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS todos (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  todo        TEXT NOT NULL,
  info        TEXT,
  status      TEXT NOT NULL DEFAULT 'todo',
  project_id  INTEGER,
  due_date    DATE,
  FOREIGN KEY ( project_id ) REFERENCES projects( id )
);

