use std::ops::{Add, Mul};
use std::sync::{Arc, Mutex};
use std::thread;

use rayon::prelude::*;

use mandelbrot::*;

const WIDTH: u16 = 500;
const HEIGHT: u16 = 500;

const FRAMERATE: f32 = 24.0;

const KEYFRAMES: [Keyframe; 3] = [
    Keyframe {
        x_center: -0.75,
        y_center: 0.0,
        x_size: 3.5,
        y_size: 3.5,
        index: 0,
    },
    Keyframe {
        x_center: -1.35,
        y_center: 0.0,
        x_size: 0.2,
        y_size: 0.2,
        index: 100,
    },
    Keyframe {
        x_center: -0.75,
        y_center: 0.0,
        x_size: 3.5,
        y_size: 3.5,
        index: 300,
    },
];

const MAX_ITER: usize = 255;

fn main() {
    let mut animation =
        Animation::new("anim.gif", WIDTH, HEIGHT, FRAMERATE).expect("Error creating animation.");

    println!("Collecting frames...");
    let frames = frames_native();
    let frames = frames_rayon();

    animation.add_frames(frames);
    animation
        .write_animation()
        .expect("Error saving animation.");
}

/// Parallel frame builder that only uses Rust threads and synchronization primitives.
pub fn frames_native() -> Vec<Frame> {
    let interpolated_frames = get_interpolated_frames(&KEYFRAMES);
    let frame_container = Arc::new(Mutex::new(Vec::with_capacity(interpolated_frames.len())));
    {
        let mut frames = frame_container.lock().unwrap();
        for _ in &interpolated_frames {
            frames.push(Frame::empty());
        }
    }

    let mut handles = vec![];
    for (i, keyframe) in interpolated_frames.iter().cloned().enumerate() {
        let frame_container = frame_container.clone();
        let handle = thread::spawn(move || {
            let pixels = draw_frame(WIDTH as u32, HEIGHT as u32, keyframe);
            let frame = Frame::from_pixels(WIDTH , HEIGHT, pixels);
            let mut frames = frame_container.lock().unwrap();
            frames[i] = frame;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let completed_frames = Arc::try_unwrap(frame_container).unwrap().into_inner().unwrap();
    completed_frames
}

/// Parallel frame builder that uses Rayon.
pub fn frames_rayon() -> Vec<Frame> {
    let interpolated_frames = get_interpolated_frames(&KEYFRAMES);
    
    interpolated_frames.par_iter().map(|keyframe| {
        let pixels = draw_frame(WIDTH as u32, HEIGHT as u32, *keyframe);
        Frame::from_pixels(WIDTH, HEIGHT, pixels)
    }).collect()
}

pub fn calc_pixel((x, y): (f32, f32)) -> Pixel {
    let c = Complex::new(x, y);
    let mut z = Complex::new(0.0, 0.0);
    let mut n = 0;

    while z.norm() <= 8192.0 && n < MAX_ITER {
        z = z * z + c;
        n += 1;
    }

    let mut intensity: f32 = 0.0;
    if n < MAX_ITER {
        let log_zn = z.norm().log2() / 2.0;
        let mu = log_zn.log2();
        intensity = (n as f32 + 1.0 - mu) / MAX_ITER as f32;
    }

    let r = intensity.powf(2.0);
    let g = intensity;
    let b = intensity.powf(0.5);

    Pixel::from_rgb(r, g, b)
}

pub fn draw_frame(width: u32, height: u32, keyframe: Keyframe) -> Vec<Pixel> {
    let mut pixels = Vec::with_capacity((width * height) as usize);

    for y in 0..height {
        for x in 0..width {
            let (x_mapped, y_mapped) = Keyframe::get_coordinate(&keyframe, x, y, width, height);
            let pixel = calc_pixel((x_mapped, y_mapped));
            pixels.push(pixel);  
        }
    }

    pixels
}

#[derive(Clone, Copy, Debug)]
struct Complex {
    x: f32,
    y: f32,
}

impl Complex {
    pub fn new(x: f32, y: f32) -> Self {
        Self {x, y}
    }

    pub fn norm(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }
}

impl Add for Complex{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {x: self.x * rhs.x - self.y * rhs.y, y: self.x * rhs.y + self.y * rhs.x}
    }
}

