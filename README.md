## Site

[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)

This is my personal website/blog/portfolio. In here, you can find the frontend/backend for the site.

For reference, it's blazing fast. To do this, we use a few ~~dirty~~ hacks:

 - Build all markdown into HTML before launching the web server as a separate command
 - Serve dynamic content, sorta
 - ~~Using Rust~~
 
### Usage

So, you want to use this site? See how it works? Be my guest.

#### With Nix

```
nix run
```

Will provide a CLI application to run the web server or build the posts into HTML content! For example, to start the webserver, we can run:

```
nix run .# -- run
```

#### Without Nix

Install Nix, then see [With Nix](#with nix).

Just kidding. If you have Rust installed, you can run:

```
cargo run
```

To access the same command-line application.

For example, let's start the webserver again:

```
cargo run -- run
```

Honestly though, you should be trying Nix!
