local M = {}

function M.show(text, color)
    print("\27[" .. color .. "m" .. text .. "\27[0m")
end

return M