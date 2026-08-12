//! Zamani Software Genome — Defining, inheriting, and evolving program genetics.

#[derive(Debug, Clone, PartialEq)]
pub enum GeneType {
    Structural,
    Behavioral,
    Metabolic,
    Adaptive,
    Quantum,
}

#[derive(Debug, Clone)]
pub struct Gene {
    pub name: String,
    pub gene_type: GeneType,
    pub expression_level: f32, // 0.0 to 1.0
    pub mutable: bool,
}

#[derive(Debug, Clone)]
pub struct SoftwareGenome {
    pub species_name: String,
    pub generation: u64,
    pub fitness_score: f32,
    pub genes: Vec<Gene>,
}

pub struct GenomeEvolutionEngine {
    genome: SoftwareGenome,
    mutation_rate: f32,
}

impl GenomeEvolutionEngine {
    pub fn new(species_name: &str, initial_genes: Vec<Gene>) -> Self {
        GenomeEvolutionEngine {
            genome: SoftwareGenome {
                species_name: species_name.to_string(),
                generation: 1,
                fitness_score: 1.0,
                genes: initial_genes,
            },
            mutation_rate: 0.05,
        }
    }

    pub fn mutate(&mut self) {
        self.genome.generation += 1;
        println!("[Genome] Evolving species '{}' to generation {}...", self.genome.species_name, self.genome.generation);
        for gene in &mut self.genome.genes {
            if gene.mutable {
                // Fluctuate expression level slightly
                gene.expression_level = (gene.expression_level + (rand_simple() * 0.2 - 0.1)).clamp(0.0, 1.0);
            }
        }
    }

    pub fn crossover(&mut self, other: &SoftwareGenome) {
        println!("[Genome] Performing genetic crossover between '{}' and '{}'", self.genome.species_name, other.species_name);
        if !other.genes.is_empty() {
            // Introduce a gene from the other genome
            self.genome.genes.push(other.genes[0].clone());
        }
        self.genome.generation += 1;
    }

    pub fn report(&self) {
        println!("=== Software Genome Report: {} (Gen {}) ===", self.genome.species_name, self.genome.generation);
        println!("Fitness Score: {:.4}", self.genome.fitness_score);
        println!("Total Genes: {}", self.genome.genes.len());
        for gene in &self.genome.genes {
            println!(" - Gene '{}' [Type: {:?}] Expression: {:.2}, Mutable: {}", gene.name, gene.gene_type, gene.expression_level, gene.mutable);
        }
    }
}

fn rand_simple() -> f32 {
    // Deterministic pseudorandom pseudo-generator for sandbox consistency
    0.53
}
