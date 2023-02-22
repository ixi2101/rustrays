
type Rgb = (u8,u8,u8);

pub trait PPM {
    fn to_disk(&self);
    fn to_stdout(&self);
    fn to_string(&self) -> String;
}

pub struct Image {
    pub data: Vec<Vec<Rgb>>,
    pub width: usize,
    pub height: usize,
}
impl Image {
    pub fn new(data: Vec<Vec<Rgb>>) -> Self {
	Image { width: data[0].len(), height: data.len(), data}

    }
}

impl PPM for Image {
    fn to_disk(&self) {
	std::fs::write("foobar.ppm", self.to_string()).unwrap();
    }

    fn to_stdout(&self) {
        println!("{}", self.to_string());
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str("P3\n");
        s.push_str(&format!("{} {}", self.width, self.height));
        s.push_str("\n255\n"); // MAX colour
        self.data
            .iter()
            .flatten()
            .map(|pxl| format!("{} {} {}", pxl.0, pxl.1, pxl.2))
            .collect::<Vec<String>>()
            .chunks(self.width)
            .for_each(|f| s.push_str(&format!("{}\n", f.join(" "))));
        s
    }
}

fn main() {
    let img: Vec<Vec<(u8, u8, u8)>> = vec![
        vec![(255, 0, 0), (0, 255, 0), (0, 0, 255)],
        vec![(255, 255, 0), (255, 255, 255), (0, 0, 0)],
    ];
    let i  = Image::new(img);
    i.to_disk();
}
