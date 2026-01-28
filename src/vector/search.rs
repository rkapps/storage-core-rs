use crate::vector::similarity::cosine_similarity;

// vector_search uses cosine_simularity to get the score. Returns truncated top_k.
pub fn vector_search<K: Clone>(
    vec_a: &[f32],
    candidates: &[(K, Vec<f32>)],
    top_k: usize,
) -> Vec<(K, f32)> {
    // Iterate the embeddings, perform cosine_similarity and return vector of id(String) and score
    let mut scores: Vec<(K, f32)> = candidates
        .into_iter()
        .map(|(id, vec)| (id.clone(), cosine_similarity(vec_a, vec)))
        .collect();

    // Sort descending by score (highest first)
    scores.sort_by(|a, b: &(K, f32)| b.1.partial_cmp(&a.1).unwrap());

    // truncate by top_k
    scores.truncate(top_k);
    scores
}

#[cfg(test)]
mod tests {
    use crate::vector::search::vector_search;


    #[test]
    fn test_vector_search_top_k() {
        let vec_a = vec![1.0, 0.0, 0.0];
        let candidates = vec![
            (1, vec![1.0, 0.0, 2.0]),
            (2, vec![1.0, 2.0, 3.0]),
            (3, vec![1.0, 3.0, 4.0]),
            (4, vec![1.0, 3.0, 5.0]),
        ];

        let results = vector_search(&vec_a, &candidates, 2);
        assert!(results.len() == 2);
        assert_eq!(results[0].0, 1);
        assert_eq!(results[1].0, 2);
        println!("{:?}", results);
    }
}
