use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneticParams {
    pub center_control_weight: f64,
    pub piece_count_weight: f64,
    pub threat_weight: f64,
    pub mobility_weight: f64,
    pub vertical_control_weight: f64,
    pub horizontal_control_weight: f64,
}

impl Default for GeneticParams {
    fn default() -> Self {
        Self {
            center_control_weight: 1.0,
            piece_count_weight: 0.5,
            threat_weight: 2.0,
            mobility_weight: 0.8,
            vertical_control_weight: 1.2,
            horizontal_control_weight: 1.0,
        }
    }
}

impl GeneticParams {
    pub fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        Self {
            center_control_weight: rng.gen_range(0.0..3.0),
            piece_count_weight: rng.gen_range(0.0..2.0),
            threat_weight: rng.gen_range(0.5..5.0),
            mobility_weight: rng.gen_range(0.0..2.0),
            vertical_control_weight: rng.gen_range(0.5..3.0),
            horizontal_control_weight: rng.gen_range(0.5..3.0),
        }
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let params: GeneticParams = serde_json::from_str(&content)?;
        Ok(params)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn random_mutation(&self, mutation_rate: f64, mutation_strength: f64) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        Self {
            center_control_weight: if rng.gen_bool(mutation_rate) {
                self.center_control_weight + (rng.gen_range(-1.0..1.0) * mutation_strength)
            } else {
                self.center_control_weight
            },
            piece_count_weight: if rng.gen_bool(mutation_rate) {
                self.piece_count_weight + (rng.gen_range(-0.5..0.5) * mutation_strength)
            } else {
                self.piece_count_weight
            },
            threat_weight: if rng.gen_bool(mutation_rate) {
                self.threat_weight + (rng.gen_range(-1.0..1.0) * mutation_strength)
            } else {
                self.threat_weight
            },
            mobility_weight: if rng.gen_bool(mutation_rate) {
                self.mobility_weight + (rng.gen_range(-0.5..0.5) * mutation_strength)
            } else {
                self.mobility_weight
            },
            vertical_control_weight: if rng.gen_bool(mutation_rate) {
                self.vertical_control_weight + (rng.gen_range(-0.5..0.5) * mutation_strength)
            } else {
                self.vertical_control_weight
            },
            horizontal_control_weight: if rng.gen_bool(mutation_rate) {
                self.horizontal_control_weight + (rng.gen_range(-0.5..0.5) * mutation_strength)
            } else {
                self.horizontal_control_weight
            },
        }
    }

    pub fn crossover(&self, other: &Self, crossover_rate: f64) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        Self {
            center_control_weight: if rng.gen_bool(crossover_rate) {
                other.center_control_weight
            } else {
                self.center_control_weight
            },
            piece_count_weight: if rng.gen_bool(crossover_rate) {
                other.piece_count_weight
            } else {
                self.piece_count_weight
            },
            threat_weight: if rng.gen_bool(crossover_rate) {
                other.threat_weight
            } else {
                self.threat_weight
            },
            mobility_weight: if rng.gen_bool(crossover_rate) {
                other.mobility_weight
            } else {
                self.mobility_weight
            },
            vertical_control_weight: if rng.gen_bool(crossover_rate) {
                other.vertical_control_weight
            } else {
                self.vertical_control_weight
            },
            horizontal_control_weight: if rng.gen_bool(crossover_rate) {
                other.horizontal_control_weight
            } else {
                self.horizontal_control_weight
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_params() {
        let params = GeneticParams::default();
        assert_eq!(params.center_control_weight, 1.0);
        assert_eq!(params.piece_count_weight, 0.5);
        assert_eq!(params.threat_weight, 2.0);
        assert_eq!(params.mobility_weight, 0.8);
        assert_eq!(params.vertical_control_weight, 1.2);
        assert_eq!(params.horizontal_control_weight, 1.0);
    }

    #[test]
    fn test_random_params() {
        let params1 = GeneticParams::random();
        let params2 = GeneticParams::random();

        // Random params should be different
        assert_ne!(params1.center_control_weight, params2.center_control_weight);
        assert_ne!(params1.threat_weight, params2.threat_weight);

        // But should be within reasonable bounds
        assert!(params1.center_control_weight >= 0.0 && params1.center_control_weight <= 3.0);
        assert!(params1.threat_weight >= 0.5 && params1.threat_weight <= 5.0);
    }

    #[test]
    fn test_save_and_load() {
        let params = GeneticParams::default();
        let temp_path = "test_params.json";

        params.save_to_file(temp_path).unwrap();
        let loaded_params = GeneticParams::load_from_file(temp_path).unwrap();

        assert_eq!(params, loaded_params);

        // Clean up
        std::fs::remove_file(temp_path).unwrap();
    }

    #[test]
    fn test_mutation() {
        let original = GeneticParams::default();
        let mutated = original.random_mutation(1.0, 0.1);

        // Mutation should change values
        assert_ne!(
            original.center_control_weight,
            mutated.center_control_weight
        );
        assert_ne!(original.threat_weight, mutated.threat_weight);
    }

    #[test]
    fn test_crossover() {
        let parent1 = GeneticParams::default();
        let parent2 = GeneticParams {
            center_control_weight: 5.0,
            piece_count_weight: 3.0,
            threat_weight: 7.0,
            mobility_weight: 4.0,
            vertical_control_weight: 6.0,
            horizontal_control_weight: 8.0,
        };

        let child = parent1.crossover(&parent2, 1.0);

        // With 100% crossover rate, child should inherit from parent2
        assert_eq!(child.center_control_weight, parent2.center_control_weight);
        assert_eq!(child.threat_weight, parent2.threat_weight);
    }
}
