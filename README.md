# PupBin : The easiest pastebin/Logging service in the west 

```
   ___            ___  _
  / _ \__ _____  / _ )(_)__
 / ___/ // / _ \/ _  / / _ \
/_/   \_,_/ .__/____/_/_//_/
         /_/

sharing, generating and storing logs and text files have never been easier!

PupBin aims to fit right into your workflow, where sharing and getting files doesn’t need you to open a different application.
```

First off, this project has only been alive for few weeks now, hence it'll be a while before this project gets the maturity one expects from a large project.

I truly am sorry for the kinda large Readme, hope this table of contents helps : 

## Table of Contents

<!-- vim-markdown-toc GFM -->

* [For developers](#Development-topics)
  * [Under the hood](#under-the-hood)
  * [Performance](#performance)
  * [Limits and resources](#limits-and-resources)
  * [Future](#future)
* [Tool Installation](#tool-installation)
  * [Using git](#using-git)
  * [Using curl](#using-curl)
  * [Using Linux package managers](#using-linux-package-managers)
  * [Windows binaries](#windows-binaries)
  * [Linux binaries](#linux-binaries)
  * [As Vim plugin](#as-vim-plugin)
* [Tool Usage](#usage)
    * [Using the pastebin](#using-the-pastebin)
    * [Using the vim plugin](#using-the-vim-plugin)
    * [Demo](#demo)
* [Building PupBin Server](#building-pupbin-backend)
* [Vim plugin Usage](#vim-plugin)
* [License](#license)

<!-- vim-markdown-toc -->


Development topics
--------------

### Under the hood 

This is THE most labour intensive project I've worked in so far. Not cus the project is hard, its because I am trying to doing it right. Like they would do in the industry. Following this, here are the points that show this project in the best of its light : 
- Uses Rust and Go with 90%+ code error coverage [i.e close to production level quality].
- Uses Postgresql with proper self configured linux systems.
- Uses seaorm to deal with postgres connections and migrations. seaorm also provides the tools to generate entitied making sure there are NO illegal transactions to the SQL databse!
- Uses redis key-value hash based store as the cache layer.
- The above all have been used with well researched and thoughtout systems design.
- Two ways to using the service (For now...more otw) : 
    - cli tool
    - vim plugin
- Uses Microsoft Azure to host backend.
    - For now all I have is a single B1 instance holding all the backend, cache and DB's [More about this down below in `Limits`].
- Uses cloudflare DNS to provide ssl/tls along with Service Protection.
- The backend in fully REST compliant, although im not exposing the REST methods as of release v1.0 [More about this down below in `Limits`]. 
- Also, do note that this is my first serious Rust project. Absolutely loved using it!
- Uses a unique Key generation service written in GO ( Note : It has drivers and such, this can be a project on it's own ). Feel free to use RKGS in your projects, If I ever get around to fixing the apparent bugs in RKGS I might even open up that API for others to use.
    - This and the main application currently communicate through UDP. As mentioned below, I plan to transfer these over to gRPC's.

### Performance 

Honestly? I haven't tested anything in terms of performance, I can say this much that it is reliable, but given the hardware I have currently hosted it I don’t have very high hopes for performance, this will get more clear in the section below.

### Limits and resources

I am a uni student with absolutely no funds at the moment, every service of this project is hosted on free tier stuff. Infact it is so bad that I couldn't even compile the app on the vm instance before hosting it, I had to attach a 4G ssd disk and use it as swap in the Azure instance to even get it compiled. To put this into perspective I am using a single B1 instance that has 1vCpu and 1G ram. This one instance already handle the API backend, cache layer, postgresql and redis. I simply can not cram more code for load balancer and rate limiter in this one instance, I simply do not have the funds to spawn more instances.

For exactly above mentioned reasons I have not exposed my API endpoints. To save it from abuse [Note : I still have abuse protection on my DNS layer, so your ddos attempts are useless].

If this project grows a lot and is being helpful to a lot of you, I simply ask for very tiny donations simply for my server costs xD and none for the development. Maybe later I can make the API endpoints as pay to use, this will come in handy for developers looking for a easy to use log collection service.

If you observe, you may question : Navin, why don’t you host this service yourself on your home network in your old hardware? Oh, that's cus I got a new ISP and these guys use CGNAT (double NAT layers) making my home network impossible to access from outside the network. The only solution I found was using a reverse proxy like ngrok, but this feels too janky and unreliable even for me [the free tier of ngrok I mean, I'm sure the paid tier is really good].

My decision of puttin the ip address behind a cloudflare dns was also because of the lack of resources, If Azure were ever to remove the free tier of something better comes along, I can switch my backend entirely without the frontend clients suffering.

### Future 

This project has a lot more to come, things I simply can't wait to implement, ANY CONTRIBUTION of ANY SORTS are welcome :).
- As for now, vim plug calls the cli tool's binary. In version 2 the plugin will do network calls on its own making it completey independent.
- More linux package manager 
- Looking at some general stability imporovements, it's not at a level I would like for this project.
- Making API endpoints public, with documentation that is.
- Load balancers (From VM isntance) and rate limiters (Fromt DNS layer).
- If time permits, i may even consider converting internal service communication so http2 using gRPC's.
- As for release 1, the database schema is simply not mature enough. It's just enough to get the job done, for version 2, there will most definetly be improvements in this regard.
- Good testing suite, with performance measures.
- Clean up service is still not coded out, this is going to be TOP PRIORITY for our next release.

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
git clone --depth=1 git@github.com:NavinShrinivas/PupBin.git ~/PupBinSources
cd ~/PupBinSources
# This script only works on linux
sudo ./install_tool.sh
```

### Using curl
```sh
# Works only on linux
curl  -sSf https://pupbin.ml/install_script | sh
```

### Using Linux package managers

As for now, I've only pacakged the build for arch linux, you can install the tool using any AUR helper. Here is the command for yay :
```sh
yay -S pupbin
```

### Windows binaries

> Note : compatibility of binaries working in your systems cant be confirmed, please preffer using other methods.

You can find the latest binaries in github releases

Soon we will try and pacakge our tool in winget and choclatey

### Linux binaries

> Note : compatibility of binaries working in your systems cant be confirmed, please preffer using other methods.

You can find the latest binaries in github releases

### As vim plugins 

Here I have provided this service through VimPlug, Would appreciate if anyone figures out other vim package managers and implement it as well.
As for now, vim plug calls the cli tool's binary. In version 2 the plugin will do network calls on its own making it completey independent.
The vim plugin as of now can only do pastes, as I am still finding a good way to fetch pastes and store it in clipboard buffer.

In terminal :
```sh
curl  -sSf https://pupbin.ml/install_script | sh
```
After above, If you use
[vim-plug](https://github.com/junegunn/vim-plug), add this line to your Vim
configuration file:
```vim
Plug 'NavinShrinivas/PupBin'
```

Usage
------

### Using the pastebin

- To make new pastes : 
```sh
# You can also use help to see the commands in terminal itself :
pupbin --help

# To make a new paste
pupbin --paste /path/to/utf-8/encoded/file
```

- To fetch a paste :
```sh
pupbin --get paste_key(usually is 5 chars long)
```

### using the vim plugin

- You can only create pastes from vim, but its a VERY usefull feature :
```vim
:CreatePaste
```
> Note : I tested this plugin only in neovim

### Demo 

- Using the tool :

- Using the vim plugin :


Building PupBin Server
----------------------

> Note: This section is for people who want to contribute and test their builds locally. Very soon we'll automate this and even make a CI/CD pipeline if the need for it rises.

### For first time builders

The pupbin backend has multiple parts that work together, for running a server instance by yourself : 
- First install the dependencies : 
    - postgresql 
    - rust toochain
    - go tool chain
    - redis server 
- start postgresql service and create a user and databse in postgresql 
- make a `.env` file in migration/ folder and fill it with this : 
```
DATABASE_URL = postgres://USER:PASSWORD@localhost/DATABASE_NAME
```
- After doing so, do the following in migrations folder :
```
cargo run -- fresh
```
- make a `.env` folder in root folder (of source) and fill it like so :
```
DATABASE_NAME = DATABASE_NAME
DATABASE_PASSWORD = PASSWORD
DATABASE_USER = USER
```
- go to RKGS folder and do `go build .` and after which `./RKGS`
- and in one more terminal do `cargo build` in root folder (of source) and after which `sudo ./target/debug/PupBin_APP-API`

### For regular builder

- Make sure postgresql and redis are active and running
- Just check if any new migrations were submitted upsteam, if so run the migration code.
- go to RKGS folder and do `go build .` and after which `./RKGS`
- and in one more terminal do `cargo build` in root folder (of source) and after which `sudo ./target/debug/PupBin_APP-API`


[License](LICENSE)
------------------

The MIT License (MIT)
Copyright (c) 2022-2023 P K Navin Shrinivas
