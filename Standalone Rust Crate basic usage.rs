//! Basic usage example for dotrepute-core
//!
//! Demonstrates how to use the library for reputation scoring.

use dotrepute_core::{
    scoring::{ScoreCalculator, MetricData, WeightConfig},
    validation, encoding, crypto,
};

fn main() {
    println!("DotRepute Core - Example Usage\n");

    // Create metric data
    let data = MetricData {
        governance_votes: 50,
        governance_proposals: 5,
        staking_amount: 1_000_000_000,
        staking_duration: 2_592_000, // 30 days
        identity_verified: true,
        identity_judgements: 2,
        community_posts: 100,
        community_upvotes: 500,
    };

    // Calculate score with default weights
    let calculator = ScoreCalculator::new();
    match calculator.calculate(&data) {
        Ok(result) => {
            println!("=== Reputation Scores ===");
            println!("Governance Score:  {}/100", result.governance_score);
            println!("Staking Score:     {}/100", result.staking_score);
            println!("Identity Score:    {}/100", result.identity_score);
            println!("Community Score:   {}/100", result.community_score);
            println!("Total Score:       {}", result.total_score);
            println!("Weighted Score:    {}/100\n", result.weighted_score);
        }
        Err(e) => eprintln!("Error calculating score: {}", e),
    }

    // Use custom weights
    let custom_weights = WeightConfig {
        governance_weight: 40,
        staking_weight: 30,
        identity_weight: 20,
        community_weight: 10,
    };

    match ScoreCalculator::with_weights(custom_weights) {
        Ok(custom_calculator) => {
            if let Ok(result) = custom_calculator.calculate(&data) {
                println!("=== With Custom Weights ===");
                println!("Weighted Score: {}/100\n", result.weighted_score);
            }
        }
        Err(e) => eprintln!("Error with custom weights: {}", e),
    }

    // Data validation
    println!("=== Data Validation ===");
    let text = "  Hello World  ";
    let normalized = validation::normalize_text(text);
    println!("Normalized: '{}' -> '{}'", text, normalized);

    // Cryptographic operations
    println!("\n=== Cryptographic Operations ===");
    let data_to_hash = b"reputation data";
    let hash = crypto::simple_hash(data_to_hash);
    let hex_hash = encoding::hex_encode(&hash);
    println!("Hash: {}", hex_hash);

    let checksum = crypto::checksum(data_to_hash);
    println!("Checksum: {}", checksum);
}
