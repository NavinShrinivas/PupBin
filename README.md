# PupBin : The easiest pastebin in the west 

   ___            ___  _
  / _ \__ _____  / _ )(_)__
 / ___/ // / _ \/ _  / / _ \
/_/   \_,_/ .__/____/_/_//_/
         /_/

sharing, generating and storing logs and text files have never been easier!



I truly am sorry for the kinda large Readme, hope this table of contents helps : 

## Table of Contents


<!-- vim-markdown-toc GFM -->

* [Tool Installation](#tool-installation)
  * [Using git](#using-git)
  * [Using Linux package managers](#using-linux-package-managers)
  * [Windows](#windows)
  * [As Vim plugin](#as-vim-plugin)
* [Building PupBin Server](#building-pupbin-backend)
* [Tool Usage](#usage)
    * [Using the pastebin](#using-the-pastebin)
    * [Options](#options)
    * [Demo](#demo)
* [Vim plugin Usage](#vim-plugin)
* [For developers](#Development-topics)
  * [Technologies](#technologies)
  * [Performance](#performance)
  * [Limits](#limits)
  * [Future](#future)
* [License](#license)

<!-- vim-markdown-toc -->

Tool Installation
-----------------

PupBin contains many part, such as : 
- backend rust api server 
- backend go cache layer
- seaorm migrations and entitity generation
- postgresql server
- Frontend tool

Here we see ways to isntall the frontend tool

### Using git 
```sh
git clone --depth=1 https://github.com/NavinShrinivas/PupBin ~/PupBinSources
cd PupBinSources
# This script only works on linux
./install_tool.sh
```

### Using Linux package managers

As for now, I've only pacakged the build for arch linux, you can isntall the tool using any AUR helper. Here is the command for yay :
```
yay -S pupbin
```
[License](LICENSE)
------------------

The MIT License (MIT)
Copyright (c) 2022-2023 P K Navin Shrinivas
