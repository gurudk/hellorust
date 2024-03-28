use std::fs::File;
use std::io::Write;
use std::ops::Range;

pub const MIN_SPAN: i32 = 8;
pub const NODE_RADIUS: i32 = 10;
pub const MARGIN: usize = 20;

pub struct SvgDraw {
    pub row: usize,
    pub col: usize,
    elements: Vec<String>,
}

impl SvgDraw {
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            row: row,
            col: row,
            elements: Vec::new(),
        }
    }

    pub fn circle(&mut self, x0: i32, y0: i32, r: i32, stroke: String, width: i32) -> &mut Self {
        self.elements.push(format!(
            r#"
            <circle cx="{x0}" cy="{y0}" r="{r}" fill="none" stroke="{stroke}" stroke-width="{width}"/>
            "#
        ));
        self
    }

    pub fn clear(&mut self) {
        self.elements.clear();
    }

    pub fn text(
        &mut self,
        x0: i32,
        y0: i32,
        font_width: i32,
        font_height: i32,
        text: String,
    ) -> &mut Self {
        let len = text.chars().count() as i32;
        let x1 = x0 - font_width * len / 2;
        let y1 = y0 + font_height / 2;
        self.elements.push(format!(
            r#"
            <text x="{x1}" y="{y1}">{text}</text>
            "#
        ));

        self
    }

    pub fn line(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        stroke: String,
        width: i32,
    ) -> &mut Self {
        self.elements.push(format!(
            r#"
            <line x1="{x1}" y1="{y1}"  x2="{x2}"  y2="{y2}" stroke="{stroke}" stroke-width="{width}
            "/>"#
        ));

        self
    }

    pub fn line_joint_circle(
        &mut self,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        r: f64,
        stroke: String,
        width: i32,
    ) -> &mut Self {
        let vec: Vec<()> = Vec::new();
        let f1 = |x: f64, y: f64| -> f64 { (x - x1) * (x - x1) + (y - y1) * (y - y1) - r * r };

        let f2 = |x: f64, y: f64| -> f64 { (x - x2) * (x - x2) + (y - y2) * (y - y2) - r * r };

        let ly = |x: f64| -> f64 { (y2 - y1) * (x - x1) / (x2 - x1) + y1 };
        let mut loss_f1_min = f64::MAX;
        let mut loss_f2_min = f64::MAX;
        let mut x_r1 = 0;
        let mut x_r2 = 0;
        let xrange = |x_begin, x_end| -> Range<i32> {
            let ix1 = x_begin as i32;
            let ix2 = x_end as i32;
            if ix1 < ix2 {
                ix1..ix2
            } else {
                ix2..ix1
            }
        };

        for i in xrange(x1, x2) {
            let f1_loss = f1(i as f64, ly(i as f64)).abs();
            if f1_loss < loss_f1_min {
                loss_f1_min = f1_loss;
                x_r1 = i;
            }

            let f2_loss = f2(i as f64, ly(i as f64)).abs();
            if f2_loss < loss_f2_min {
                loss_f2_min = f2_loss;
                x_r2 = i;
            }
        }

        // println!(
        //     "P1:{},{},P2:{},{}",
        //     x_r1,
        //     ly(x_r1 as f64) as i32,
        //     x_r2,
        //     ly(x_r2 as f64) as i32,
        // );

        self.line(
            x_r1,
            ly(x_r1 as f64) as i32,
            x_r2,
            ly(x_r2 as f64) as i32,
            stroke,
            width,
        );

        self
    }

    pub fn render(&self, file_name: String) -> std::io::Result<()> {
        let mut f = File::create(&file_name)?;
        let width = self.col;
        let height = self.row;
        let mut svgcontent = String::new();
        for element in &self.elements {
            svgcontent.push_str(element);
        }

        let svgbody = format!(
            r#"
            <svg width="{width}" height="{height}" version="1.1" xmlns="http://www.w3.org/2000/svg"><rect width="100%" height="100%" fill="white"/>
            {svgcontent}
            </svg>
            "#
        );

        f.write_all(svgbody.as_bytes())?;

        Ok(())
    }

    fn body(&mut self) -> &mut Self {
        self
    }
}
