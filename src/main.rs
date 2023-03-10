use indicatif::{ProgressBar, ProgressBarIter};
use rand::Rng;
use rustrays::vec3;
type Rgb = vec3::Vec3;


pub trait PPM {
    fn to_disk(&self);
    fn to_stdout(&self);
    fn to_string(&self) -> String;
}

pub struct Image {
    pub data: Vec<Vec<Rgb>>,
    pub width: usize,
    pub height: usize,
    pub total_pixels: u64,
}
impl Image {
    pub fn new(data: Vec<Vec<Rgb>>) -> Self {
        let total = data.iter().flatten().count() as u64;
        Image {
            width: data[0].len(),
            height: data.len(),
            total_pixels: total,
            data,
        }
    }

    pub fn sample(dim: u64) -> Self {
        // Produce 256x256 image
        let pb = ProgressBar::new(dim).with_message("Generating sample image");
        let mut rng = rand::thread_rng();
        let mut img: Vec<Vec<Rgb>> = Vec::new();
        for _ in 0..=dim {
            pb.inc(1);
            let mut row: Vec<Rgb> = Vec::new();
            for _ in 0..=dim {
                row.push(
		    vec3::Vec3{
			x: fastrand::f32(),
			y: fastrand::f32(),
			z: fastrand::f32(),
		    });
            }
            img.push(row);
        }
        Image::new(img)
    }
}

impl PPM for Image {
    fn to_disk(&self) {
        if self.total_pixels > 500000000 / 4 {
        }
        std::fs::write("foobar.ppm", self.to_string()).unwrap();
    }

    fn to_stdout(&self) {
        println!("{}", self.to_string());
    }

    fn to_string(&self) -> String {
        let pb = ProgressBar::new(self.total_pixels).with_message("Saving to .ppm");

        let mut s = String::new();
        s.push_str("P3\n");
        s.push_str(&format!("{} {}", self.width, self.height));
        s.push_str("\n255\n"); // MAX colour

        for px in self.data.iter().flatten() {
            pb.inc(1);
            s.push_str(&px.colour_fmt());
	}

        s
    }
}

fn main() {
    let i = Image::sample(1000);
    i.to_disk();
}
