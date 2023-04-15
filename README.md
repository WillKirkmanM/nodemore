# nodemore
💾 Nodemore Recursively Searches Directories for Unused Projects

![NODEMORE](https://user-images.githubusercontent.com/98240335/232202453-078ba9ef-081e-4ff5-b576-eb27c3230102.png)

Contents
========

 * [Why?](#why)
 * [Installation](#installation)
 * [Usage](#usage)
 * [Configuration](#configuration) 

Why?
========
<p align="center">
  <img 
  src="https://i.redd.it/tfugj4n3l6ez.png"
  width=330
  height=237
   />
</p>

NodeJS has a **horrible** way of dealing with dependencies. One that intentionally aims to strip you of any functionality other than programming in Javascript. I found myself trying to install a small 100GB game only to find that I couldn't due to half of my 1TB hard disk being taken over by `node_modules` (That was a horror game in itself).

and it's in Rust.

> If linux is free if you don't value your time, 
> then Javascript is free if you don't value your space.

Installation
========
## Windows
Head to the [releases](https://github.com/WillKirkmanM/nodemore/releases/) page and download the [MSI build](https://github.com/WillKirkmanM/nodemore/releases/download/v1.0.0/nodemore-1.0.0-x86_64.msi). From there, install it like any other program

## Linux / MacOS
```
wget https://github.com/WillKirkmanM/nodemore/releases/download/v1.0.0/nodemore -O ~/.local/bin
```

Usage
======== 
To start the program, simply run 
```
$ nodemore
```

`nodemore` was built with functionality in mind. Every feature has been meticulously chosen for ease of use and simplicity. Enjoy!

```shell
Usage: nodemore [OPTIONS]

Options:
  -p, --path <PATH>            Path to Search [default: .]
  -t, --time <TIME>            Time Frame [default: "1 week"]
  -a, --prompt                 Prompt Before Deletion
  -s, --show-size              Whether to show file sizes (Slow)
  -v, --verbosity <VERBOSITY>  Verbosity Level [default: 0]
  -h, --help                   Print help
  -V, --version                Print version
```

Configuration
========
Nodemore has a custom `nodemore.yml` config file that you can use instead of the command line (you can find an [example here](https://github.com/WillKirkmanM/nodemore/blob/main/nodemore.yml))

```yml
# nodemore.yml
cleaning:
  time: "1 day"
  path: "."
```

As present, the configurations will be stored in the `cleaning` section holding both a `time` and a `path` option. These would be equivalent to running:
```
$ nodemore -t "1 day"
```
and
```
$ nodemore -p "."
```
