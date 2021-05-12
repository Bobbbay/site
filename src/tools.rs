// This file contains most helpful tools that the project provides - e.g. the automated pandoc
// conversion, etc.

use anyhow::Result;

use std::fs::read_dir;

use pandoc::OutputKind;
use pandoc::PandocOption::{Css, Standalone};

/// `compile_posts` compiles all `.md` files in /posts of the project directory into HTML files.
pub(crate) fn build_posts() -> Result<()> {
    // /posts/*.md
    // /serve/blog/*.html

    let posts = read_dir("./posts/")?;

    for post in posts {
        // e.g. ./posts/writing-this-blog.md
        let post = post?.path();

        let mut output = post.clone();
        output.pop();
        output.pop();
        output.push("serve");
        output.push("blog");
        output.push(post.clone().file_stem().unwrap());
        // e.g. ./serve/blog/writing-this-blog/index.html

        // First, let's ensure that the directory we're creating in exists and is empty
        if output.exists() {
            std::fs::remove_dir_all(output.clone())?;
        }

        std::fs::create_dir_all(output.clone())?;

        output.push("index.html");

        let mut pandoc = pandoc::new();
        pandoc.add_input(&post);
        pandoc.set_output(OutputKind::File(output.clone()));
        pandoc.add_option(Css("/css/index.css".to_string()));
        pandoc.add_option(Standalone);
        pandoc.execute().unwrap();

        println!("> Completed {:?} > {:?}", post, output);
    }

    println!(">> Operation completed successfully.");

    Ok(())
}
