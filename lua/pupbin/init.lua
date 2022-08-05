-- Imports the plugin's additional Lua modules.
local create_paste = require("pupbin.create_paste")

-- Creates an object for the module. All of the module's
-- functions are associated with this object, which is
-- returned when the module is called with `require`.
local M = {}

-- Routes calls made to this module to functions in the
-- plugin's other modules.
M.create_paste = create_paste.paste_function

return M
