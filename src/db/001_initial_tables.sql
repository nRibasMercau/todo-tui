CREATE TABLE IF NOT EXISTS projects (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  name        TEXT NOT NULL,
  archived    INTEGER NOT NULL DEFAULT 0,
  created_at  DATE NOT NULL DEFAULT CURRENT_DATE
);


CREATE TABLE IF NOT EXISTS todos (
  id            INTEGER PRIMARY KEY AUTOINCREMENT,
  todo          TEXT NOT NULL,
  info          TEXT,
  status        TEXT NOT NULL DEFAULT 'todo',
  project_id    INTEGER,
  due_date      DATE, 
  created_at    DATE NOT NULL DEFAULT CURRENT_DATE,
  completed_at  DATE,
  FOREIGN KEY ( project_id ) REFERENCES projects( id ) ON DELETE CASCADE
);

