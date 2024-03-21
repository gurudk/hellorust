use std::collections::HashSet;
fn main() {
    println!("Hello, world!");
    let canvas: [[Option<char>; 80]; 100] = [[None; 80]; 100];
    let mut data: Vec<Vec<Option<char>>> = vec![vec![Some('-'); 80]; 100];
    data[0][0] = Some('d');

    println!("{:?}", to_row_str(&data[0]));
    let mut test: String = String::from("");
    test.push('3');
    println!("{:?}", test);

    println!("=========================================");

    let mut asc = AsciiDraw::new(100, 80, ' ');
    asc.line(20, 20, 20, 40, '-')
        .line(40, 40, 20, 40, '-')
        .circle(40, 40, 10, 'd')
        .circle(40, 40, 20, 'e')
        // .line(0, 0, 20, 20, '\\')
        .line(40, 0, 20, 20, '/')
        .line(0, 0, 20, 20, '\\')
        .render();

    println!("{:?}", asc.data[0]);
}

struct AsciiDraw {
    row: usize,
    column: usize,
    data: Vec<Vec<Option<char>>>,
    bg_char: char,
    factor: usize,
}

impl AsciiDraw {
    fn new(row: usize, column: usize, bgc: char) -> Self {
        Self {
            row: row,
            column: column,
            data: vec![vec![Some(bgc); column]; row],
            bg_char: bgc,
            factor: 2,
        }
    }

    fn line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, c: char) -> &mut Self {
        if x1 == x2 {
            if y1 < y2 {
                for y in y1..y2 {
                    self.data[y][x1] = Some(c);
                }
            } else {
                for y in y2..y1 {
                    self.data[y][x1] = Some(c);
                }
            }
        } else if y1 == y2 {
            if x1 < x2 {
                for x in x1..x2 {
                    self.data[y1][x] = Some(c);
                }
            } else {
                for x in x2..x1 {
                    self.data[y1][x] = Some(c);
                }
            }
        } else {
            let mut hs = HashSet::new();
            if x1 < x2 {
                for x in x1..x2 {
                    let y = (y2 - y1) * (x - x1) / (x2 - x1) + y1;
                    if !hs.contains(&y) {
                        hs.insert(y);
                        self.data[y][x] = Some(c);
                    }
                }
            } else {
                for x in x2..x1 {
                    let y = (y2 - y1) * (x1 - x) / (x1 - x2) + y1;
                    if !hs.contains(&y) {
                        hs.insert(y);
                        self.data[y][x] = Some(c);
                    }
                }
            }
        }

        self
    }

    fn circle(&mut self, x0: usize, y0: usize, r: usize, c: char) -> &mut Self {
        for x in x0 - r..=x0 + r {
            let xx = (x as i32 - x0 as i32);
            let y1 = (((r * r) as i32 - xx * xx) as f64).sqrt() as usize + y0;
            let y2 = y0 - (((r * r) as i32 - xx * xx) as f64).sqrt() as usize;
            self.data[y1][x] = Some(c);
            self.data[y2][x] = Some(c);
        }

        self
    }

    fn to_row_str(&self, row_no: usize) -> Box<String> {
        let mut str: String = String::from("");
        for c in &self.data[row_no] {
            str.push(c.unwrap());
        }

        Box::new(str)
    }

    fn render(&self) {
        for i in 0..self.row {
            println!("{}", self.to_row_str(i));
        }
    }
}

fn to_row_str(vec: &Vec<Option<char>>) -> Box<String> {
    let mut str: String = String::from("");
    for c in vec {
        match c {
            Some(cc) => {
                str.push(*cc);
            }
            None => str.push(' '),
        }
    }

    Box::new(str)
}
