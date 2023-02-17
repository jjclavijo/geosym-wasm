extern crate realfft;
extern crate wasm_bindgen;

use realfft::{RealFftPlanner, num_complex::Complex};
use wasm_bindgen::prelude::*;

fn convert_to_real(buffer: &[Complex<f32>]) -> &[f32] {
    // This is where I'd put the rationale for why this `unsafe` block
    // upholds the guarantees that I must ensure. Too bad I
    // copy-and-pasted from Stack Overflow without reading this comment!
    // 
    // Este comentario es genial!
    //
    // Se supone que no hay problema en hacer esto porque en todos lados la
    // documentación dice que Complex<f32> ocupa exactamente siempre igual
    // la memoria que 2 f32 y eso está garantizado.
    // 
    // Además siempre hay que tener un lugar del código donde pueda haber
    // errores de memoria, la vida es mucho mas divertida así.
    unsafe {
        let ptr = buffer.as_ptr() as *mut f32;
        let len = buffer.len();

        std::slice::from_raw_parts(ptr, len * 2)
    }
}

#[wasm_bindgen]
pub fn compute_fft(n: usize, datos: Vec<f32>) -> Vec<f32> {
  // Perform a forward FFT of size 1234
  
  let mut planner = RealFftPlanner::new();
  let fft = planner.plan_fft_forward(n);
  
  let mut buffer = datos;
  let mut output = fft.make_output_vec();  
  let result = fft.process(&mut buffer, &mut output);
  assert_eq!(result.is_ok(),true);
  convert_to_real(&*output).to_vec()
}
