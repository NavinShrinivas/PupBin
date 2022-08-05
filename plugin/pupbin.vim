" Title:        pupbin
" Description:  The plugin interface for pupbin
" Last Change:  5 AUG 2022
" Maintainer:   Navin Shrinivas <https://github.com/NavinShrinivas>

" Prevents the plugin from being loaded multiple times. If the loaded
" variable exists, do nothing more. Otherwise, assign the loaded
" variable and continue running this instance of the plugin.
if exists("g:loaded_pupbin")
    finish
endif
let g:loaded_pupbin = 1

" Defines a package path for Lua. This facilitates importing the
" Lua modules from the plugin's dependency directory.
let s:lua_rocks_deps_loc =  expand("<sfile>:h:r") . "lua/pupbin/"
exe "lua package.path = package.path .. ';" . s:lua_rocks_deps_loc . "/lua-?/init.lua'"

" Exposes the plugin's functions for use as commands in Neovim.
command! -nargs=0 CreatePaste lua require("pupbin").create_paste()
