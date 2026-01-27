use crate::vector::similarity::cosine_similarity;

// vector_search uses cosine_simularity to get the score. Returns truncated top_k.
pub fn vector_search(
    vec_a: &[f32],
    embeddings: &[(String, Vec<f32>)],
    top_k: usize,
) -> Vec<(String, f32)> {
    // Iterate the embeddings, perform cosine_similarity and return vector of id(String) and score
    let mut scores: Vec<(String, f32)> = embeddings
        .iter()
        .map(|(id, vec)| (id.clone(), cosine_similarity(vec_a, vec)))
        .collect();

    // Sort descending by score (highest first)
    scores.sort_by(|a, b: &(String, f32)| b.1.partial_cmp(&a.1).unwrap());

    // truncate by top_k
    scores.truncate(top_k);
    scores
}
