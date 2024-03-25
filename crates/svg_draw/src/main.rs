use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let mut svg = String::from("test");

    svg.push_str("sdfsf");
    svg.push_str("\"");

    println!("{}", svg);

    let width = 300;
    let height = 500;
    let mut svgcontent = String::from("");
    let c = circle(50, 100, 20, String::from("red"), 5);
    let l = line(50, 100, 100, 50, String::from("red"), 5);
    svgcontent.push_str(&c);
    svgcontent.push_str(&l);
    let svgbody = format!(
        r#"
        <svg width="{width}" height="{height}" version="1.1" xmlns="http://www.w3.org/2000/svg"><rect width="100%" height="100%" fill="white"/>
        {svgcontent}
        </svg>
        "#
    );

    println!("{}", svgbody);

    let mut file = File::create("tree_image.svg")?;
    file.write_all(svgbody.as_bytes())?;
    Ok(())
}

fn circle(x0: usize, y0: usize, r: usize, stroke: String, width: usize) -> String {
    format!(r#"
    <circle cx="{x0}" cy="{y0}" r="{r}" stroke="{stroke}" stroke-width="{width}"/>
    "#)
}

fn line(x1: usize, y1: usize, x2: usize, y2: usize, stroke: String, width: usize) -> String {
    format!(
        r#"
        <line x1="{x1}" y1="{y1}"  x2="{x2}"  y2="{y2}" stroke="{stroke}" stroke-width="{width}"/>
        "#
    )
}
