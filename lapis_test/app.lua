local lapis = require("lapis")
local app = lapis.Application()
local sqlite3 = require("lsqlite3")
app:enable("etlua")
app.layout = require "views.layout"

local function get_db()
  return sqlite3.open("database.db")
end


app:get("index", "/", function(self)
  self.foods = {
    "apple",
    "pear",
    "coriander",
    "gherkin"
  }

  return { render = "index" }
end)

return app
