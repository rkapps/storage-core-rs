

// cosine_similarity - dot product provides the cosing of the anglet between 2 vectors
pub fn cosine_similarity(vec_a: &[f32], vec_b: &[f32]) -> f32 {

    let mag_a = magnitude(vec_a);
    let mag_b = magnitude(vec_b);

    let dot_prod = vec_a.iter().zip(vec_b).map(|(x,y)| x*y).sum::<f32>();
    if mag_a == 0.0 || mag_b == 0.0 {
        return 0.0;
    }
    
    dot_prod / (mag_a /  mag_b)
}

// magniture - calculates the magnitude(length) of the vector
fn magnitude(vec: &[f32]) -> f32 {
    vec.iter().map(|e| e.powf(2.0)).sum::<f32>().sqrt()
}