local sqlite3 = require "lsqlite3"

function get_db()
  return sqlite3.open("database.db")
end

function select(stmt)
  local db = get_db()
  return assert( db:prepare(stmt) ):nrows()
end
