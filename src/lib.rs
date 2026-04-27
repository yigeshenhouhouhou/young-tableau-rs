pub mod partition;
pub mod diagram;
pub mod cell;
pub mod tableau;
pub mod dimension;
pub mod skew;
pub mod lr_rule;
pub mod enumeration;

pub use partition::Partition;
pub use diagram::YoungDiagram;
pub use cell::{Cell, Content, Hook};
pub use tableau::{YoungTableau, StandardYoungTableau};
pub use dimension::{su_dimension, count_standard_tableaux};
pub use skew::{SkewShape, SemiStandardSkewTableau};
pub use lr_rule::{lr_coefficient, tensor_product_decompose};
pub use enumeration::enumerate_standard_tableaux;

#[cfg(test)]
mod tests {
    use super::*;
    use num_rational::Rational64;

    #[test]
    fn test_partition_basics() {
        let p = Partition::new(vec![3, 2, 1]).unwrap();
        assert_eq!(p.size(), 6);
        assert_eq!(p.len(), 3);
        assert_eq!(p.get(0), Some(3));
        assert_eq!(p.get(3), None);
    }

    #[test]
    fn test_partition_validation() {
        assert!(Partition::new(vec![3, 2, 1]).is_ok());
        assert!(Partition::new(vec![1, 2, 3]).is_err());
        assert!(Partition::new(vec![]).is_ok());
    }

    #[test]
    fn test_partition_contains() {
        let nu = Partition::new(vec![3, 2, 1]).unwrap();
        let lam = Partition::new(vec![2, 1]).unwrap();
        assert!(nu.contains(&lam));
        
        let bad = Partition::new(vec![3, 3]).unwrap();
        assert!(!nu.contains(&bad));
    }

    #[test]
    fn test_generate_partitions() {
        let p4 = Partition::generate_all(4);
        assert_eq!(p4.len(), 5);
    }

    #[test]
    fn test_young_diagram_cells() {
        let diag = YoungDiagram::from_partition(Partition::new(vec![2, 1]).unwrap());
        let cells: Vec<_> = diag.cells().collect();
        assert_eq!(cells.len(), 3);
        assert!(Cell::new(1, 1).is_in(&diag));
        assert!(!Cell::new(2, 2).is_in(&diag));
    }

    #[test]
    fn test_hook_lengths() {
        let diag = YoungDiagram::from_partition(Partition::new(vec![3, 2, 1]).unwrap());
        let hooks: Vec<_> = diag.hook_lengths().collect();
        assert_eq!(hooks, vec![5, 3, 1, 3, 1, 1]);
    }

    #[test]
    fn test_conjugate() {
        let shape = YoungDiagram::from_partition(Partition::new(vec![3, 2, 1]).unwrap());
        let conj = shape.conjugate();
        assert_eq!(conj.partition().parts(), &[3, 2, 1]);

        let shape2 = YoungDiagram::from_partition(Partition::new(vec![4, 2]).unwrap());
        let conj2 = shape2.conjugate();
        assert_eq!(conj2.partition().parts(), &[2, 2, 1, 1]);
    }

    #[test]
    fn test_su3_dimensions() {
        let singlet = YoungDiagram::from_partition(Partition::new(vec![1, 1, 1]).unwrap());
        assert_eq!(su_dimension(&singlet, 3), Rational64::new(1, 1));

        let fund = YoungDiagram::from_partition(Partition::new(vec![1]).unwrap());
        assert_eq!(su_dimension(&fund, 3), Rational64::new(3, 1));

        let octet = YoungDiagram::from_partition(Partition::new(vec![2, 1]).unwrap());
        assert_eq!(su_dimension(&octet, 3), Rational64::new(8, 1));

        let decuplet = YoungDiagram::from_partition(Partition::new(vec![3]).unwrap());
        assert_eq!(su_dimension(&decuplet, 3), Rational64::new(10, 1));

        let sextet = YoungDiagram::from_partition(Partition::new(vec![2]).unwrap());
        assert_eq!(su_dimension(&sextet, 3), Rational64::new(6, 1));
    }

    #[test]
    fn test_count_standard_tableaux() {
        let shape = YoungDiagram::from_partition(Partition::new(vec![2, 1]).unwrap());
        assert_eq!(count_standard_tableaux(&shape), Rational64::new(2, 1));

        let shape2 = YoungDiagram::from_partition(Partition::new(vec![3]).unwrap());
        assert_eq!(count_standard_tableaux(&shape2), Rational64::new(1, 1));

        let shape3 = YoungDiagram::from_partition(Partition::new(vec![2, 2]).unwrap());
        assert_eq!(count_standard_tableaux(&shape3), Rational64::new(2, 1));
    }

    #[test]
    fn test_standard_young_tableau_validation() {
        use std::collections::HashMap;
        let shape = YoungDiagram::from_partition(Partition::new(vec![2, 1]).unwrap());
        
        let mut filling = HashMap::new();
        filling.insert(Cell::new(1, 1), 1);
        filling.insert(Cell::new(1, 2), 3);
        filling.insert(Cell::new(2, 1), 2);
        assert!(StandardYoungTableau::try_new(shape.clone(), filling).is_ok());

        let mut bad = HashMap::new();
        bad.insert(Cell::new(1, 1), 2);
        bad.insert(Cell::new(1, 2), 1);
        bad.insert(Cell::new(2, 1), 3);
        assert!(StandardYoungTableau::try_new(shape.clone(), bad).is_err());
    }

    #[test]
    fn test_enumerate_standard_tableaux() {
        let shape = YoungDiagram::from_partition(Partition::new(vec![2, 1]).unwrap());
        let tableaux = enumerate_standard_tableaux(&shape);
        assert_eq!(tableaux.len(), 2);
    }

    #[test]
    fn test_lr_coefficient_basic() {
        let lam = Partition::new(vec![1]).unwrap();
        let mu = Partition::new(vec![2]).unwrap();
        
        let nu1 = Partition::new(vec![3]).unwrap();
        let nu2 = Partition::new(vec![2, 1]).unwrap();
        let nu_bad = Partition::new(vec![2]).unwrap();
        
        assert_eq!(lr_coefficient(&lam, &mu, &nu1), 1);
        assert_eq!(lr_coefficient(&lam, &mu, &nu2), 1);
        assert_eq!(lr_coefficient(&lam, &mu, &nu_bad), 0);
    }

    #[test]
    fn test_tensor_product_decompose() {
        let lam = Partition::new(vec![2]).unwrap();
        let mu = Partition::new(vec![2]).unwrap();
        let result = tensor_product_decompose(&lam, &mu);
        
        let mut map: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        for (nu, c) in &result {
            map.insert(format!("{}", nu), *c);
        }
        
        assert_eq!(map.get("[4]"), Some(&1));
        assert_eq!(map.get("[3, 1]"), Some(&1));
        assert_eq!(map.get("[2, 2]"), Some(&1));
    }

    #[test]
    fn test_qcd_fundamental_tensor_product() {
        let lam = Partition::new(vec![1]).unwrap();
        let mu = Partition::new(vec![1]).unwrap();
        let result = tensor_product_decompose(&lam, &mu);
        
        assert_eq!(result.len(), 2);
        let mut dims: Vec<_> = result.iter()
            .map(|(nu, c)| {
                let diag = YoungDiagram::from_partition(nu.clone());
                let dim = su_dimension(&diag, 3);
                (dim, *c)
            })
            .collect();
        dims.sort();
        
        assert_eq!(dims[0], (Rational64::new(3, 1), 1));
        assert_eq!(dims[1], (Rational64::new(6, 1), 1));
    }

    #[test]
    fn test_qcd_triple_product() {
        let lam = Partition::new(vec![1, 1]).unwrap();
        let mu = Partition::new(vec![1]).unwrap();
        let result = tensor_product_decompose(&lam, &mu);
        
        let mut has_octet = false;
        let mut has_singlet = false;
        for (nu, c) in &result {
            let diag = YoungDiagram::from_partition(nu.clone());
            let dim = su_dimension(&diag, 3);
            if dim == Rational64::new(8, 1) && *c > 0 { has_octet = true; }
            if dim == Rational64::new(1, 1) && *c > 0 { has_singlet = true; }
        }
        assert!(has_octet, "3̄ ⊗ 3 should contain octet");
        assert!(has_singlet, "3̄ ⊗ 3 should contain singlet");
    }

    #[test]
    fn test_lr_jdt_basic() {
        let lam = Partition::new(vec![1]).unwrap();
        let mu = Partition::new(vec![2]).unwrap();
        
        let nu1 = Partition::new(vec![3]).unwrap();
        let nu2 = Partition::new(vec![2, 1]).unwrap();
        let nu_bad = Partition::new(vec![2]).unwrap();
        
        assert_eq!(lr_coefficient_jdt(&lam, &mu, &nu1), lr_coefficient(&lam, &mu, &nu1));
        assert_eq!(lr_coefficient_jdt(&lam, &mu, &nu2), lr_coefficient(&lam, &mu, &nu2));
        assert_eq!(lr_coefficient_jdt(&lam, &mu, &nu_bad), lr_coefficient(&lam, &mu, &nu_bad));
    }

    #[test]
    fn test_lr_jdt_tensor_product() {
        let lam = Partition::new(vec![2]).unwrap();
        let mu = Partition::new(vec![2]).unwrap();
        
        let jdt = tensor_product_decompose_jdt(&lam, &mu);
        let slow = tensor_product_decompose(&lam, &mu);
        
        assert_eq!(jdt.len(), slow.len());
        for ((nu_j, c_j), (nu_s, c_s)) in jdt.iter().zip(slow.iter()) {
            assert_eq!(nu_j, nu_s);
            assert_eq!(c_j, c_s);
        }
    }

    #[test]
    fn test_lr_jdt_qcd() {
        let lam = Partition::new(vec![1]).unwrap();
        let mu = Partition::new(vec![1]).unwrap();
        
        let jdt = tensor_product_decompose_jdt(&lam, &mu);
        let slow = tensor_product_decompose(&lam, &mu);
        
        assert_eq!(jdt.len(), slow.len());
        for ((nu_j, c_j), (nu_s, c_s)) in jdt.iter().zip(slow.iter()) {
            assert_eq!(nu_j, nu_s);
            assert_eq!(c_j, c_s);
        }
    }

    #[test]
    fn test_lr_jdt_larger() {
        let lam = Partition::new(vec![2, 1]).unwrap();
        let mu = Partition::new(vec![2, 1]).unwrap();
        
        let jdt = tensor_product_decompose_jdt(&lam, &mu);
        let slow = tensor_product_decompose(&lam, &mu);
        
        assert_eq!(jdt.len(), slow.len());
        for ((nu_j, c_j), (nu_s, c_s)) in jdt.iter().zip(slow.iter()) {
            assert_eq!(nu_j, nu_s);
            assert_eq!(c_j, c_s);
        }
    }
}