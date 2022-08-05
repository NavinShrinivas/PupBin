local M = {}

function M.paste_function()
    local path = vim.api.nvim_buf_get_name(0)
    if path == "" then
        print("No active open buffer's found :(")
        return
    end
    print("Pasting contents of : ",path)
    os.execute("rm -f /tmp/pupbin-vim-plugin > /dev/null")
    os.execute("touch /tmp/pupbin-vim-plugin")
    local commands = "pupbin --paste "..path.." > /tmp/pupbin-vim-plugin";
    print(commands)
    local output  = os.execute(commands)
    if  output~=true and output ~= 0 then
        print("Something went wrong or pupbin is not installed!") --Needs better error handling
        print(output)
    end
    local file = io.open("/tmp/pupbin-vim-plugin", "r")
    if file ~= nil then
        print("Output from tool : ",file:read("*a"))
    else
        print("Something went wrong with fetching output!")
    end

end

return M
