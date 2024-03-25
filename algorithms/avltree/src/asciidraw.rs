use std::collections::HashSet;
fn main() {
    println!("Hello, world!");
    let canvas: [[Option<char>; 80]; 100] = [[None; 80]; 100];
    let mut data: Vec<Vec<Option<char>>> = vec![vec![Some('-'); 80]; 100];
    data[0][0] = Some('d');

    println!("=========================================");

    let mut asc = AsciiDraw::new(100, 120, ' ');
    asc.line(20, 20, 20, 40, '-')
        .line(40, 40, 20, 40, '-')
        .circle(40, 40, 10, 'd')
        .circle(40, 40, 8, '.')
        .line(0, 0, 20, 20, '\\')
        .line(40, 0, 20, 20, '/')
        .line(0, 0, 15, 20, '.')
        .line(10, 80, 100, 10, '=')
        .draw_box(50, 70, 2, String::from("abcd"))
        .draw_circle(50, 80, 3, '.', 5, String::from("3"))
        .draw_box(60, 70, 1, String::from("4"))
        .draw_box_center(20, 20, 1, String::from("8"))
        // .draw_circle(20, 20, 3, '#', 5, String::from("3"))
        .render();

    // println!("{:?}", asc.data[0]);
    let mut str = String::from("");

    // str.push(box_char_for_dirs(10));
    str.push(box_char_for_dirs(12));
    str.push(box_char_for_dirs(12));
    str.push(box_char_for_dirs(12));
    // str.push(box_char_for_dirs(6));

    let mut down_str = String::from("");
    down_str.push(box_char_for_dirs(9));
    down_str.push(box_char_for_dirs(12));
    down_str.push(box_char_for_dirs(12));
    down_str.push(box_char_for_dirs(12));
    down_str.push(box_char_for_dirs(5));

    let mut mid_str = String::from("");

    mid_str.push(box_char_for_dirs(3));

    println!("=========================================");
    println!("/{}\\", str);
    println!("{}abc{}", &mid_str, &mid_str);
    println!("{down_str}");
}

pub struct AsciiDraw {
    row: usize,
    column: usize,
    data: Vec<Vec<Option<char>>>,
    bg_char: char,
    factor: usize,
}

impl AsciiDraw {
    pub fn new(row: usize, column: usize, bgc: char) -> Self {
        Self {
            row: row,
            column: column,
            data: vec![vec![Some(bgc); column]; row],
            bg_char: bgc,
            factor: 2,
        }
    }

    pub fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, c: char) -> &mut Self {
        if x1 == x2 {
            if y1 < y2 {
                for y in y1..y2 {
                    self.data[y as usize][x1 as usize] = Some(c);
                }
            } else {
                for y in y2..y1 {
                    self.data[y as usize][x1 as usize] = Some(c);
                }
            }
        } else if y1 == y2 {
            if x1 < x2 {
                for x in x1..x2 {
                    self.data[y1 as usize][x as usize] = Some(c);
                }
            } else {
                for x in x2..x1 {
                    self.data[y1 as usize][x as usize] = Some(c);
                }
            }
        } else {
            let mut hs = HashSet::new();
            if x1 < x2 {
                for x in x1..x2 {
                    let y = (y2 - y1) * (x - x1) / (x2 - x1) + y1;
                    if !hs.contains(&y) {
                        hs.insert(y);
                        self.data[y as usize][x as usize] = Some(c);
                    }
                }
            } else {
                for x in x2..x1 {
                    let y = (y2 - y1) * (x1 - x) / (x1 - x2) + y1;
                    if !hs.contains(&y) {
                        hs.insert(y);
                        self.data[y as usize][x as usize] = Some(c);
                    }
                }
            }
        }

        self
    }

    pub fn circle(&mut self, x0: i32, y0: i32, r: i32, c: char) -> &mut Self {
        for x in x0 - r..=x0 + r {
            let xx = x - x0;
            let y1 = ((r * r - xx * xx) as f64 / 3.6).sqrt() as i32 + y0;
            let y2 = y0 - ((r * r - xx * xx) as f64 / 3.6).sqrt() as i32;
            self.data[y1 as usize][x as usize] = Some(c);
            self.data[y2 as usize][x as usize] = Some(c);
        }

        self
    }

    pub fn draw_circle(
        &mut self,
        x0: i32,
        y0: i32,
        r: i32,
        c: char,
        text_width: usize,
        text: String,
    ) -> &mut Self {
        //draw circle edge
        for x in x0 - r..=x0 + r {
            let xx = x - x0;
            let y1 = ((r * r - xx * xx) as f64 / 3.6).sqrt() as i32 + y0;
            let y2 = y0 - ((r * r - xx * xx) as f64 / 3.6).sqrt() as i32;
            self.data[y1 as usize][x as usize] = Some(c);
            self.data[y2 as usize][x as usize] = Some(c);
        }

        //draw content
        let len = text.chars().count();
        let mut rows = len / text_width;
        let r = len % text_width;
        if r > 0 {
            rows += 1;
        }

        let x_content = (x0 as f64 - (len as f64 - 1.0) / 2.0) as usize;
        println!("x0:{},x_content:{},text_width", x0, x_content);
        let y_content = y0 as usize - rows / 2;
        let mut chars: Vec<_> = text.chars().collect();
        for i in 0..chars.len() {
            let col_idx = i / text_width;
            let row_idx = i % text_width;
            self.data[y_content + col_idx][x_content + row_idx] = Some(chars[i].clone());
        }

        self
    }

    pub fn draw_box_center(
        &mut self,
        x1: i32,
        y1: i32,
        text_width: i32,
        text: String,
    ) -> &mut Self {
        let len = text.chars().count() as i32;
        let mut rows = len / text_width;
        let r = len % text_width;
        if r > 0 {
            rows += 1;
        }

        let x0 = x1 - 2 - text_width / 2;
        let y0 = y1 - 1 - rows / 2;

        self.draw_box(x0, y0, text_width, text);

        self
    }

    pub fn draw_box(&mut self, x0: i32, y0: i32, text_width: i32, text: String) -> &mut Self {
        let len = text.chars().count() as i32;
        let mut rows = len / text_width;
        let r = len % text_width;
        if r > 0 {
            rows += 1;
        }

        let chars: Vec<_> = text.chars().collect();
        //draw top line
        let x_begin = x0;
        let x_end = x0 + text_width + 4;
        for i in x_begin..x_end {
            if i == x_begin {
                self.data[y0 as usize][i as usize] = Some(box_char_for_dirs(10));
            } else if i == x_end - 1 {
                self.data[y0 as usize][i as usize] = Some(box_char_for_dirs(6));
            } else {
                self.data[y0 as usize][i as usize] = Some(box_char_for_dirs(12));
            }
        }

        //draw bottom line
        let y_bottom = y0 + rows + 2;
        for i in x_begin..x_end {
            if i == x_begin {
                self.data[(y_bottom - 1) as usize][i as usize] = Some(box_char_for_dirs(9));
            } else if i == x_end - 1 {
                self.data[(y_bottom - 1) as usize][i as usize] = Some(box_char_for_dirs(5));
            } else {
                self.data[(y_bottom - 1) as usize][i as usize] = Some(box_char_for_dirs(12));
            }
        }

        //draw left line
        let y_begin = y0 + 1;
        let y_end = y_begin + rows;
        for i in y_begin..y_end {
            self.data[i as usize][x0 as usize] = Some(box_char_for_dirs(3));
        }

        //draw right line
        for i in y_begin..y_end {
            self.data[i as usize][(x_end - 1) as usize] = Some(box_char_for_dirs(3));
        }

        //draw content
        let x_content = x0 + 2;
        let y_content = y0 + 1;
        // let x_content_end = x_content + width;
        // let y_content_end = y_content + rows;
        for i in 0..chars.len() {
            let col_idx = i as i32 / text_width;
            let row_idx = i as i32 % text_width;
            self.data[(y_content + col_idx) as usize][(x_content + row_idx) as usize] =
                Some(chars[i].clone());
        }

        self
    }

    pub fn to_row_str(&self, row_no: usize) -> Box<String> {
        let mut str: String = String::from("");
        for c in &self.data[row_no] {
            str.push(c.unwrap());
        }

        Box::new(str)
    }

    pub fn render(&self) {
        for i in 0..self.row {
            println!("{}", self.to_row_str(i));
        }
    }
}
///////////////////////////////////////////////////////////////////////////
// Unicode box-drawing characters

const UP: u8 = 0b0001;
const DOWN: u8 = 0b0010;
const LEFT: u8 = 0b0100;
const RIGHT: u8 = 0b1000;

const BOX_CHARS: &'static [(char, u8)] = &[
    ('╵', UP),
    ('│', UP | DOWN),
    ('┤', UP | DOWN | LEFT),
    ('├', UP | DOWN | RIGHT),
    ('┼', UP | DOWN | LEFT | RIGHT),
    ('┘', UP | LEFT),
    ('└', UP | RIGHT),
    ('┴', UP | LEFT | RIGHT),
    // No UP:
    ('╷', DOWN),
    ('┐', DOWN | LEFT),
    ('┌', DOWN | RIGHT),
    ('┬', DOWN | LEFT | RIGHT),
    // No UP|DOWN:
    ('╶', LEFT),
    ('─', LEFT | RIGHT),
    // No LEFT:
    ('╴', RIGHT),
    // No RIGHT:
    (' ', 0),
];

fn box_char_for_dirs(dirs: u8) -> char {
    for &(c, d) in BOX_CHARS {
        if dirs == d {
            return c;
        }
    }
    panic!("no box character for dirs: {:b}", dirs);
}

fn dirs_for_box_char(ch: char) -> Option<u8> {
    for &(c, d) in BOX_CHARS {
        if c == ch {
            return Some(d);
        }
    }
    None
}

fn add_dirs(old_ch: char, new_dirs: u8) -> char {
    let old_dirs = dirs_for_box_char(old_ch).unwrap_or(0);
    box_char_for_dirs(old_dirs | new_dirs)
}
