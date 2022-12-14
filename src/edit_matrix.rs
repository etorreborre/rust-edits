use crate::costs::Cost::*;
use crate::costs::*;
use crate::matrix::*;

pub fn create_edit_matrix(costs: &impl Costs<char>, str1: String, str2: String) -> Matrix<Cost> {
    let mut matrix = init_matrix(str1.len() + 1, str2.len() + 1, NoAction(0));
    let coordinates = cartesian(str1.len() + 1, str2.len() + 1);

    for ij in coordinates {
        let (i, j) = ij;
        let new_cost: Cost = if i == 0 {
            Insertion(j)
        } else if j == 0 {
            Deletion(i)
        } else {
            match cost_of(costs, str1.clone(), str2.clone(), i, j, &matrix) {
                Some(c) => c,
                _ => NoAction(0),
            }
        };
        matrix.set_value(i, j, new_cost);
    }

    matrix
}

// Return the cartesian product of n x m elements, 0-indexed
fn cartesian(n: usize, m: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    for i in 0..n {
        for j in 0..m {
            result.push((i, j));
        }
    }
    result
}

// compute the cost of going from as1[i] to as2[j], knowing the existing costs
//  (i-1, j-1) (i-1, j)
//  (i, j-1)   (i, j)
//
// going from (i-1, j) to (i, j) means that we delete as1[i]
// going from (i-1, j-1) to (i, j) means that we substitute as1[i] with as2[j]
// going from (i, j-1) to (i, j) means that we insert as2[j]
pub fn cost_of(
    costs: &impl Costs<char>,
    str1: String,
    str2: String,
    i: usize,
    j: usize,
    matrix: &Matrix<Cost>,
) -> Option<Cost> {
    let i1 = i - 1;
    let j1 = j - 1;
    let i1j = matrix.get_value(i1, j)?;
    let i1j1 = matrix.get_value(i1, j1)?;
    let ij1 = matrix.get_value(i, j1)?;
    let v1 = str1.chars().nth(i1)?;
    let v2 = str2.chars().nth(j1)?;

    let result = costs.lower_cost(
        &v1,
        &v2,
        ij1.cost() + 1,                             // insertion
        i1j.cost() + 1,                             // suppression
        i1j1.cost() + if v1 == v2 { 0 } else { 1 }, // substitution
    ); // insertion
       // in case of a substitution if the resulting cost of (i, j) is the same as (i-1, j-1)
       // this means that we have substituted the same letter and it is the same as doing no action
    match result {
        Substitution(_) => {
            if i1j1.cost() == result.cost() {
                Some(NoAction(result.cost()))
            } else {
                Some(result)
            }
        }
        _ => Some(result),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cartesian() {
        assert_eq!(
            cartesian(2, 3),
            vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2)]
        );
    }
    #[test]
    fn test_create_edit_matrix() {
        assert_eq!(
            create_edit_matrix(&levenshtein_costs(), "hello".to_string(), "hey".to_string()),
            Matrix {
                rows: vec![
                    vec![Insertion(0), Insertion(1), Insertion(2), Insertion(3)],
                    vec![Deletion(1), NoAction(0), Insertion(1), Insertion(2)],
                    vec![Deletion(2), Deletion(1), NoAction(0), Insertion(1)],
                    vec![Deletion(3), Deletion(2), Deletion(1), Substitution(1)],
                    vec![Deletion(4), Deletion(3), Deletion(2), Substitution(2)],
                    vec![Deletion(5), Deletion(4), Deletion(3), Substitution(3)]
                ]
            }
        );
    }
}
