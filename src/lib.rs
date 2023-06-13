
use std::error::Error;
use std::f64::consts::PI;


use rustfft::{FftPlanner, num_complex::Complex};
use rayon::prelude::*;

pub enum WaveletType {
    Ricker,
    Morlet,
    Custom,
}

pub struct Wavelet {
    pub wavelet_type: WaveletType,
    pub scales: Vec<f64>,
    pub data: Vec<Vec<Complex<f64>>>, // Some wavelet types can be complex valued.
}

impl Wavelet {
    pub fn new(wavelet_type: WaveletType, scales: Vec<f64>, input: Vec<f64>) -> Wavelet {

        // Initializing the data matrix with just one entry.
        // Just a precaution in case there's no match for wavelet_type.
        let mut data = vec![vec![Complex{re:0.0, im:0.0}]];

        match wavelet_type {
            WaveletType::Ricker => {
                if let Ok(result) = scales.iter().map(|i| {
                    Self::ricker(&input, *i)
                }).collect() {data = result};
            },
            WaveletType::Morlet => {
                data = vec![vec![Complex{re:0.0, im:0.0}]];
                todo!()
            },
            WaveletType::Custom => {},
        }

        // Returning a Wavelet struct with the calculated wavelet matrix.:#![warn()]
        Wavelet {
            wavelet_type: wavelet_type,
            scales: scales,
            data: data,
        }
    }

    pub fn ricker(x: &Vec<f64>, sigma: f64) -> Result<Vec<Complex<f64>>, Box<dyn Error>> {

        let data: Vec<Complex<f64>> = x.iter().map(|i| {
            Complex {
                re: {
                    (2.0 / ((3.0 * sigma).sqrt() * PI.powf(0.25))) *
                    (1.0 - (i / sigma).powi(2)) *
                    ((-(i.powi(2) / (2.0 * sigma.powi(2)))).exp())
                }, im: 0.0}
            })
            .collect();
        
        Ok(data)
    }

    pub fn morlet() {todo!();}
}


pub struct Cwt {
    pub wavelet: Wavelet,
    pub input: Vec<Complex<f64>>,
    pub output: Vec<Vec<Complex<f64>>>,
}

impl Cwt {
    /* 
    pub fn calculate() {

    }
    */
    pub fn pad_vector(mut vec1: Vec<Complex<f64>>, mut vec2: Vec<Complex<f64>>) -> Result<(Vec<Complex<f64>>, Vec<Complex<f64>>), Box<dyn Error>> {

        let pad_num = vec1.len() - vec2.len();
        let pad_vec: Vec<Complex<f64>>;

        // Checking if and where to pad:
        // No padding if pad_num = 0.
        // if vec1 > vec2: pad vec2.
        // else vec1 must be smaller, so pad it.
        if pad_num == 0 {
            return Ok((vec1, vec2));
        } else if vec1.len() > vec2.len() {
            pad_vec = vec![Complex{re:0.0, im:0.0}; pad_num];
            vec2.extend_from_slice(&pad_vec);
            return Ok((vec1, vec2));
        } else {
            pad_vec = vec![Complex{re:0.0, im:0.0}; pad_num];
            vec1.extend_from_slice(&pad_vec);
            return Ok((vec1, vec2));
        } 
    }
    
    pub fn mult_vector(vec1: Vec<Complex<f64>>, vec2: Vec<Complex<f64>>) -> Result<Vec<Complex<f64>>, Box<dyn Error>> {
   
        let vec3: Vec<Complex<f64>> = vec1.iter()
            .zip(vec2.iter())
            .map(|(&v1, &v2)| {
                Complex{re: (v1.re * v2.re), im: (v1.im * v2.im)}
            }).collect();

        Ok(vec3)
    }
    
}
