//! Self-Modifying Programs — runtime code adaptation
//! Programs observe their own behavior and propose modifications through deliberation.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// A code mutation — a proposed change to a program
#[derive(Debug, Clone)]
pub struct Mutation {
    pub id: u64,
    pub description: String,
    pub old_code: String,
    pub new_code: String,
    pub confidence: f64,
    pub reason: MutationReason,
    pub proposed_by: String,
    pub approved: bool,
    pub applied: bool,
    pub reverted: bool,
}

#[derive(Debug, Clone)]
pub enum MutationReason {
    Performance { metric: String, before: f64, after: f64 },
    Correctness { error: String },
    Adaptation { context_change: String },
    Optimization { opportunity: String },
}

/// Runtime observation of program behavior
#[derive(Debug, Clone)]
pub struct Observation {
    pub metric: String,
    pub value: f64,
    pub timestamp_nanos: u64,
    pub context: String,
}

/// A self-modifying program
pub struct SelfModifyingProgram {
    pub code: String,
    pub mutations: Vec<Mutation>,
    pub checkpoints: Vec<Checkpoint>,
    pub observations: Vec<Observation>,
    pub adaptation_budget: u32,
    pub max_mutations_per_cycle: u32,
    pub min_confidence_for_apply: f64,
    next_mutation_id: u64,
}

#[derive(Debug, Clone)]
pub struct Checkpoint {
    pub code: String,
    pub mutation_count: usize,
    pub timestamp_nanos: u64,
    pub performance_snapshot: HashMap<String, f64>,
}

impl SelfModifyingProgram {
    pub fn new(initial_code: &str) -> Self {
        Self {
            code: initial_code.to_string(), mutations: vec![],
            checkpoints: vec![Checkpoint {
                code: initial_code.to_string(), mutation_count: 0,
                timestamp_nanos: now(), performance_snapshot: HashMap::new(),
            }],
            observations: vec![], adaptation_budget: 100,
            max_mutations_per_cycle: 5, min_confidence_for_apply: 0.7,
            next_mutation_id: 0,
        }
    }

    /// Observe runtime behavior
    pub fn observe(&mut self, metric: &str, value: f64, context: &str) {
        self.observations.push(Observation {
            metric: metric.to_string(), value, timestamp_nanos: now(),
            context: context.to_string(),
        });
    }

    /// Propose a mutation
    pub fn propose_mutation(&mut self, description: &str, new_code: &str, confidence: f64, reason: MutationReason, proposed_by: &str) -> u64 {
        let id = self.next_mutation_id;
        self.next_mutation_id += 1;
        self.mutations.push(Mutation {
            id, description: description.to_string(),
            old_code: self.code.clone(), new_code: new_code.to_string(),
            confidence, reason, proposed_by: proposed_by.to_string(),
            approved: false, applied: false, reverted: false,
        });
        id
    }

    /// Apply a mutation (if confidence meets threshold)
    pub fn apply_mutation(&mut self, mutation_id: u64) -> bool {
        if self.adaptation_budget == 0 { return false; }
        let applied_this_cycle = self.mutations.iter().filter(|m| m.applied && !m.reverted).count() as u32;
        if applied_this_cycle >= self.max_mutations_per_cycle { return false; }

        if let Some(m) = self.mutations.get_mut(mutation_id as usize) {
            if m.confidence >= self.min_confidence_for_apply && !m.applied {
                self.code = m.new_code.clone();
                m.applied = true;
                m.approved = true;
                self.adaptation_budget -= 1;
                return true;
            }
        }
        false
    }

    /// Revert the last applied mutation
    pub fn revert_last(&mut self) -> bool {
        for m in self.mutations.iter_mut().rev() {
            if m.applied && !m.reverted {
                self.code = m.old_code.clone();
                m.reverted = true;
                return true;
            }
        }
        false
    }

    /// Save a checkpoint for rollback
    pub fn save_checkpoint(&mut self) {
        let perf: HashMap<String, f64> = self.observations.iter()
            .filter(|o| o.timestamp_nanos > now() - 60_000_000_000) // last 60s
            .map(|o| (o.metric.clone(), o.value))
            .collect();
        self.checkpoints.push(Checkpoint {
            code: self.code.clone(),
            mutation_count: self.mutations.iter().filter(|m| m.applied).count(),
            timestamp_nanos: now(),
            performance_snapshot: perf,
        });
    }

    /// Rollback to a specific checkpoint
    pub fn rollback_to_checkpoint(&mut self, checkpoint_idx: usize) -> bool {
        if let Some(cp) = self.checkpoints.get(checkpoint_idx) {
            self.code = cp.code.clone();
            return true;
        }
        false
    }

    /// Get adaptation statistics
    pub fn stats(&self) -> AdaptationStats {
        let applied = self.mutations.iter().filter(|m| m.applied && !m.reverted).count();
        let reverted = self.mutations.iter().filter(|m| m.reverted).count();
        let pending = self.mutations.iter().filter(|m| !m.applied).count();
        AdaptationStats {
            total_mutations: self.mutations.len(),
            applied, reverted, pending,
            adaptation_budget_remaining: self.adaptation_budget,
            code_length: self.code.len(),
            checkpoints: self.checkpoints.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AdaptationStats {
    pub total_mutations: usize,
    pub applied: usize,
    pub reverted: usize,
    pub pending: usize,
    pub adaptation_budget_remaining: u32,
    pub code_length: usize,
    pub checkpoints: usize,
}

fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observe_and_mutate() {
        let mut prog = SelfModifyingProgram::new("return sorted(data)");
        prog.observe("latency_ms", 150.0, "normal load");
        assert_eq!(prog.observations.len(), 1);
    }

    #[test]
    fn test_apply_mutation() {
        let mut prog = SelfModifyingProgram::new("slow_sort()");
        let id = prog.propose_mutation(
            "use builtin sorted", "sorted(data)", 0.9,
            MutationReason::Optimization { opportunity: "builtin is faster".to_string() },
            "optimizer"
        );
        assert!(prog.apply_mutation(id));
        assert_eq!(prog.code, "sorted(data)");
    }

    #[test]
    fn test_low_confidence_rejected() {
        let mut prog = SelfModifyingProgram::new("fn()");
        let id = prog.propose_mutation("risky change", "risky()", 0.3,
            MutationReason::Optimization { opportunity: "maybe".to_string() }, "novice");
        assert!(!prog.apply_mutation(id));
        assert_eq!(prog.code, "fn()");
    }

    #[test]
    fn test_revert() {
        let mut prog = SelfModifyingProgram::new("v1");
        let id = prog.propose_mutation("upgrade", "v2", 0.9,
            MutationReason::Performance { metric: "speed".to_string(), before: 100.0, after: 50.0 }, "opt");
        prog.apply_mutation(id);
        assert_eq!(prog.code, "v2");
        assert!(prog.revert_last());
        assert_eq!(prog.code, "v1");
    }

    #[test]
    fn test_checkpoint_rollback() {
        let mut prog = SelfModifyingProgram::new("original");
        prog.save_checkpoint();
        prog.propose_mutation("change", "modified", 0.9,
            MutationReason::Adaptation { context_change: "new env".to_string() }, "agent");
        let id = 1; // second mutation
        prog.apply_mutation(id);
        assert_eq!(prog.code, "modified");
        assert!(prog.rollback_to_checkpoint(0));
        assert_eq!(prog.code, "original");
    }

    #[test]
    fn test_budget_limits() {
        let mut prog = SelfModifyingProgram::new("code");
        prog.adaptation_budget = 2;
        for i in 0..3 {
            let id = prog.propose_mutation(&format!("m{}", i), "new", 0.9,
                MutationReason::Optimization { opportunity: "x".to_string() }, "a");
            prog.apply_mutation(id);
        }
        assert_eq!(prog.stats().applied, 2);
    }
}
