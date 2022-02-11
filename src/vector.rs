pub fn sub_vector(lhs: &Vec<f32>, rhs: &Vec<f32>) -> Vec<f32> {
    lhs.iter().zip(rhs.iter()).map(|(l, r)| l - r ).collect()
}

pub fn add_vector(lhs: &Vec<f32>, rhs: &Vec<f32>) -> Vec<f32> {
    lhs.iter().zip(rhs.iter()).map(|(l, r)| l + r ).collect()
}

pub fn dot_product(lhs: &Vec<f32>, rhs: &Vec<f32>) -> f32 {
    lhs.iter().zip(rhs.iter()).map(|(l, r)| l * r ).sum()
}

pub fn length(vector: &Vec<f32>) -> f32 {
    f32::sqrt(vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2])
}

pub fn normalize(vector: &Vec<f32>) -> Vec<f32> {
    vector.iter().map(|c| c / length(vector)).collect()
}

pub fn scale(vector: &Vec<f32>, scale: f32) -> Vec<f32> {
    vector.iter().map(|c| c * scale).collect()
}

pub fn reflect(i: &Vec<f32>, n: &Vec<f32>) -> Vec<f32> {
    let i_dot_n = dot_product(i, n) * 2.;
    sub_vector(i, &scale(n, i_dot_n))
}

// THIS ONLY WORKS ON 3D VECTORS LOL
pub fn norm(v: &Vec<f32>) -> f32 {
    v.iter().map(|c| c * c * c).sum::<f32>().cbrt()
}