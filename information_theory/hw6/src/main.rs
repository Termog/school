use plotters::prelude::*;

use libbch::Bch;
use errch::Errch;

mod errch {
    use super::introduce_error;
    use rand::distributions::Bernoulli;
    use rand::prelude::*;
    use rand_chacha::ChaCha20Rng;
    pub struct Errch {
        // rng: ThreadRng,
        rng: ChaCha20Rng,
        bin: Bernoulli,
        // prob: f64,
    }

    impl Errch {
        pub fn new(seed: u64,prob: f64) -> Self {
            Self {
                // rng: thread_rng(),
                rng: ChaCha20Rng::seed_from_u64(seed),
                bin: Bernoulli::new(prob).unwrap(),
                // prob,
            }
        }
        pub fn pass(&mut self, message: Vec<u8>) -> Vec<u8> {
            let mut errors = Vec::new();
            for i in 0..(message.len()*8) {
                let err = self.bin.sample(&mut self.rng);
                // let err = self.rng.gen::<f64>() <  self.prob;
                if err {
                    errors.push(i);
                }
            }
            introduce_error(&message, &errors)
        }
    }
}

fn main() {
    let n: i32 = 63;
    let m = (n + 1).ilog2();
    let t = 2;
    // println!("{}",m);
    let bch = Bch::new(m.try_into().unwrap(), t, None);
    let message: Vec<u8> = vec![0, 0, 0, 0b11];
    println!("Input message: {:?}",message);
    let ecc = bch.get_ecc(&message);
    println!("ecc: {:?}",ecc);
    let mut err_message = introduce_error(&message, &[20, 27]);
    println!("Message with errors {:?}", err_message);
    let errors = bch.get_errors(&err_message, &ecc).unwrap();
    println!("Errors found {:?}", errors);
    bch.decode_from_errors(&mut err_message, &errors);
    println!("Corrected message {:?}", &err_message);
    assert_eq!(message, err_message);
    monte_carlo();
}

fn monte_carlo() {
    let points = plot_shit(63,4);
    let points2 = plot_shit(31,4);
    let image = BitMapBackend::new("graphs/1.png",(600,400)).into_drawing_area();
    image.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&image)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0.0..3e-2, 0.0..0.3)
        // .build_cartesian_2d(-3e-2..0.0, -1.0..0.0)
        .unwrap();
    // chart.draw_series(LineSeries::new(points.iter().map(|(x,y)| (-x,-y)),RED).point_size(2)).unwrap();
    // chart.draw_series(LineSeries::new(points2.iter().map(|(x,y)| (-x,-y)),BLUE).point_size(2)).unwrap();
    chart.draw_series(LineSeries::new(points.into_iter().filter(|(x,_)| *x > 0.001),RED).point_size(2)).unwrap().label("BCH(63, 4)").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    chart.draw_series(LineSeries::new(points2.into_iter().filter(|(x,_)| *x > 0.001),BLUE).point_size(2)).unwrap().label("BCH(31, 4)").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));
    chart.configure_mesh().x_desc("Error in Channel").y_desc("Error in BCH").draw().unwrap();
    chart.configure_series_labels()
    .border_style(&BLACK)
    .background_style(&WHITE.mix(0.8))
    .draw()
    .unwrap();
}

fn plot_shit(n: i32,t: i32) -> Vec<(f64,f64)> {
    let m = (n + 1).ilog2();
    // let m = n;
    // let t = 8;
    let sample_size = 1000;
    let prob_step: f64 = 1e-3;
    let prob_max: f64 = 3e-2;
    let bch = Bch::new(m.try_into().unwrap(), t, None);
    let mut points2 = Vec::new();
    for j in 0..((prob_max/prob_step) as u64) {
        let j = j as f64*prob_step;
        let mut error_prob = 0.0;
        for i in 0..sample_size {
            let mut channel = Errch::new(i,j);
            let message: Vec<u8> = vec![15, 35, 254, 128];
            let encoded = bch.encode(&message);
            let passed = channel.pass(encoded.clone());
            let decoded = match bch.decode(passed.clone()) {
                Ok(dec) => dec,
                Err(dec) => {
                    // println!("\nmessage: {:?}",message);
                    // println!("encoded: {:?}",encoded);
                    // println!("passed: {:?}",passed);
                    // println!("decoded: {:?}",dec);
                    dec
                }
            };
            let err = count_errors(&message,&decoded);
            let prob = err as f64 / message.len() as f64;
            error_prob += prob;
        }
        points2.push((j,error_prob as f64/sample_size as f64));
    }
    // println!("{:?}",points2);
    points2
}

fn count_errors(original: &[u8], decoded: &[u8]) -> u8 {
    let mut err_count = 0;
    let n = original.len();
    for i in 0..n {
        for j in 0..8 {
            if original[i] & 1 << j != decoded[i] & 1 << j {
                err_count += 1;
            }
        }
    }
    err_count
}

fn introduce_error(message: &[u8], errors: &[usize]) -> Vec<u8> {
    let mut new: Vec<u8> = message.to_vec();
    for error in errors {
        let mask = 1 << error % 8;
        let byte = error / 8;
        new[byte] ^= mask;
    }
    new
}

#[cfg(test)]

mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha20Rng;
    use rand_distr::{Bernoulli, Distribution};

    use super::*;
    #[test]
    fn count_errors_test() {
        assert_eq!(3,count_errors(&vec![0,0,0],&vec![8,4,32]));
    }
    #[test]
    fn test_bernuli() {
        let mut vec1 = Vec::new();
        let mut rng = ChaCha20Rng::seed_from_u64(2);
        let bin =  Bernoulli::new(3e-9).unwrap();
        for _ in 0..100 {
            vec1.push(bin.sample(&mut rng));
        }
        let mut vec2 = Vec::new();
        let mut rng = ChaCha20Rng::seed_from_u64(2);
        let bin =  Bernoulli::new(3e-9).unwrap();
        for _ in 0..100 {
            vec2.push(bin.sample(&mut rng));
        }
        assert_eq!(vec1,vec2);
    }
}
