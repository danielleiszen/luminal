use crate::prelude::*;

#[test]
fn test_index_select_per_row() {
    let mut graph = Graph::new();
    
    // Create a 3x4 matrix
    let matrix = graph.tensor((3, 4)).set([
        [1.0, 2.0, 3.0, 4.0],  // Row 0
        [5.0, 6.0, 7.0, 8.0],  // Row 1  
        [9.0, 10.0, 11.0, 12.0], // Row 2
    ]);
    
    // Select column indices for each row: [2, 0, 3]
    // Row 0, col 2 -> 3.0
    // Row 1, col 0 -> 5.0  
    // Row 2, col 3 -> 12.0
    let indices = graph.tensor(3).set([2.0, 0.0, 3.0]);
    
    let result = matrix.select(indices).retrieve();
    
    graph.execute();
    
    assert_eq!(result.data(), vec![3.0, 5.0, 12.0]);
}