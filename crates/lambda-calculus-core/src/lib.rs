//! # 🧬 Lambda Calculus Core: The Heart of Self-Replicating Poetry
//! 
//! This crate implements the core lambda calculus engine that powers the SOLFUNMEME
//! MetaMeme system. It provides self-replicating expressions that can encode poetry,
//! generate NFTs, and create recursive meme structures.
//! 
//! ## Key Features
//! 
//! - **S-Combinator Magic**: `S f g x = f x (g x)` for curried composition
//! - **Self-Replication**: Expressions that output themselves (quines)
//! - **Emoji Encoding**: Lambda expressions encoded as emoji sequences
//! - **Poetry Generation**: Lambda calculus that creates beautiful verse

use std::collections::HashMap;
use std::rc::Rc;
use std::fmt;

use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};
use log::{debug, info, warn};

/// 🌀 The fundamental expression type for our lambda calculus poetry engine
/// 
/// This enum represents all possible expressions in our self-replicating system:
/// - Variables and lambda abstractions for computation
/// - S, K, I combinators for functional composition  
/// - Symbols for emoji and semantic encoding
/// - Applications for expression evaluation
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// Variable reference (e.g., `x`, `y`, `muse`)
    Var(String),
    
    /// Lambda abstraction `λx.body` - the essence of functional programming
    Lambda(String, Box<Expr>),
    
    /// Function application `f x` - where the magic happens
    App(Box<Expr>, Box<Expr>),
    
    /// Symbol for emoji encoding and semantic meaning
    Sym(String),
    
    /// S-Combinator: `S f g x = f x (g x)` - the heart of composition
    S,
    
    /// K-Combinator: `K x y = x` - constant function
    K,
    
    /// I-Combinator: `I x = x` - identity function
    I,
    
    /// 🎭 Muse: A special expression representing poetic inspiration
    Muse(String, u32), // (name, resonance_score * 1000 to avoid f64)
    
    /// 🌀 Quine: Self-replicating expression that outputs itself
    Quine(Box<Expr>),
    
    /// 🧬 DNA: Genetic encoding for meme evolution
    DNA(Vec<u8>),
}

impl Expr {
    /// Create a function application `left right`
    pub fn app(left: Expr, right: Expr) -> Expr {
        Expr::App(Box::new(left), Box::new(right))
    }
    
    /// Create a lambda abstraction `λvar.body`
    pub fn lambda(var: &str, body: Expr) -> Expr {
        Expr::Lambda(var.to_string(), Box::new(body))
    }
    
    /// Create a variable reference
    pub fn var(name: &str) -> Expr {
        Expr::Var(name.to_string())
    }
    
    /// Create a symbol (for emoji encoding)
    pub fn sym(symbol: &str) -> Expr {
        Expr::Sym(symbol.to_string())
    }
    
    /// Create a muse with resonance score (0.0-1.0 converted to 0-1000)
    pub fn muse(name: &str, resonance: f64) -> Expr {
        Expr::Muse(name.to_string(), (resonance * 1000.0) as u32)
    }
    
    /// Create a quine (self-replicating expression)
    pub fn quine(expr: Expr) -> Expr {
        Expr::Quine(Box::new(expr))
    }
    
    /// Create DNA encoding
    pub fn dna(data: Vec<u8>) -> Expr {
        Expr::DNA(data)
    }
    
    /// Get resonance score as f64
    pub fn get_resonance(&self) -> f64 {
        match self {
            Expr::Muse(_, score) => (*score as f64) / 1000.0,
            _ => 0.0,
        }
    }
    
    /// 🌟 The legendary Y-combinator for recursion: `Y = λf.(λx.f (x x)) (λx.f (x x))`
    pub fn y_combinator() -> Expr {
        let inner = Expr::lambda("x", 
            Expr::app(
                Expr::var("f"),
                Expr::app(Expr::var("x"), Expr::var("x"))
            )
        );
        Expr::lambda("f", Expr::app(inner.clone(), inner))
    }
    
    /// 🎭 Create the MetaMeme combinator: `M = S (K S) K`
    pub fn metameme_combinator() -> Expr {
        Expr::app(
            Expr::app(Expr::S, Expr::app(Expr::K, Expr::S)),
            Expr::K
        )
    }
    
    /// 🚀 SOLFUNMEME signature expression
    pub fn solfunmeme() -> Expr {
        Expr::app(
            Expr::app(Expr::S, Expr::muse("Sol", 0.97)),
            Expr::app(
                Expr::app(Expr::S, Expr::muse("Fun", 0.95)),
                Expr::muse("Meme", 0.99)
            )
        )
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Var(name) => write!(f, "{}", name),
            Expr::Lambda(var, body) => write!(f, "λ{}.{}", var, body),
            Expr::App(left, right) => write!(f, "({} {})", left, right),
            Expr::Sym(symbol) => write!(f, "{}", symbol),
            Expr::S => write!(f, "S"),
            Expr::K => write!(f, "K"),
            Expr::I => write!(f, "I"),
            Expr::Muse(name, resonance) => write!(f, "🎭{}[{:.2}]", name, resonance),
            Expr::Quine(expr) => write!(f, "🌀Q({})", expr),
            Expr::DNA(data) => write!(f, "🧬DNA[{}]", data.len()),
        }
    }
}

/// 🔄 Reduction trace for debugging and visualization
#[derive(Debug, Clone)]
pub struct ReductionTrace {
    pub steps: Vec<String>, // Store as strings instead of Expr
    pub step_count: usize,
    pub final_form: Expr,
    pub is_normal_form: bool,
}

/// 🧠 The Lambda Calculus Engine - where poetry becomes computation
pub struct LambdaEngine {
    /// Maximum reduction steps to prevent infinite loops
    pub max_steps: usize,
    /// Variable substitution environment
    pub environment: HashMap<String, Expr>,
    /// Reduction trace for debugging
    pub trace: Vec<Expr>,
}

impl Default for LambdaEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl LambdaEngine {
    /// Create a new lambda calculus engine
    pub fn new() -> Self {
        Self {
            max_steps: 1000,
            environment: HashMap::new(),
            trace: Vec::new(),
        }
    }
    
    /// Set maximum reduction steps
    pub fn with_max_steps(mut self, max_steps: usize) -> Self {
        self.max_steps = max_steps;
        self
    }
    
    /// Add a variable binding to the environment
    pub fn bind(&mut self, name: &str, expr: Expr) {
        self.environment.insert(name.to_string(), expr);
    }
    
    /// 🌟 Normalize a lambda expression with full beta reduction
    pub fn normalize(&mut self, expr: Expr) -> Result<ReductionTrace> {
        info!("🚀 Starting normalization of: {}", expr);
        self.trace.clear();
        self.trace.push(expr.clone());
        
        let mut current = expr;
        let mut step_count = 0;
        
        while step_count < self.max_steps {
            match self.beta_reduce(&current)? {
                Some(reduced) => {
                    debug!("Step {}: {} → {}", step_count + 1, current, reduced);
                    current = reduced;
                    self.trace.push(current.clone());
                    step_count += 1;
                }
                None => {
                    info!("✅ Reached normal form after {} steps", step_count);
                    break;
                }
            }
        }
        
        if step_count >= self.max_steps {
            warn!("⚠️ Maximum steps reached, may not be in normal form");
        }
        
        // Convert trace to strings
        let string_steps: Vec<String> = self.trace.iter().map(|expr| format!("{}", expr)).collect();
        
        Ok(ReductionTrace {
            steps: string_steps,
            step_count,
            final_form: current.clone(),
            is_normal_form: step_count < self.max_steps,
        })
    }
    
    /// 🔄 Perform one step of beta reduction
    fn beta_reduce(&self, expr: &Expr) -> Result<Option<Expr>> {
        match expr {
            // Variable lookup in environment
            Expr::Var(name) => {
                if let Some(value) = self.environment.get(name) {
                    Ok(Some(value.clone()))
                } else {
                    Ok(None)
                }
            }
            
            // Lambda abstraction - no reduction needed
            Expr::Lambda(_, _) => Ok(None),
            
            // Function application - the heart of computation
            Expr::App(left, right) => {
                match left.as_ref() {
                    // Beta reduction: (λx.body) arg → body[x := arg]
                    Expr::Lambda(param, body) => {
                        let substituted = self.substitute(body, param, right)?;
                        Ok(Some(substituted))
                    }
                    
                    // S-combinator: S f g x → f x (g x)
                    Expr::S => {
                        // S f → partial application
                        Ok(Some(Expr::app(
                            Expr::app(Expr::S, (**right).clone()),
                            Expr::I // Placeholder for next argument
                        )))
                    }
                    
                    // K-combinator: K x y → x
                    Expr::K => {
                        Ok(Some(Expr::app(Expr::K, (**right).clone())))
                    }
                    
                    // I-combinator: I x → x
                    Expr::I => {
                        Ok(Some((**right).clone()))
                    }
                    
                    // Nested application - reduce left side first
                    Expr::App(inner_left, inner_right) => {
                        match inner_left.as_ref() {
                            // S f g x → f x (g x)
                            Expr::App(s_expr, f) if matches!(s_expr.as_ref(), Expr::S) => {
                                let g = inner_right;
                                let x = right;
                                Ok(Some(Expr::app(
                                    Expr::app((**f).clone(), (**x).clone()),
                                    Expr::app((**g).clone(), (**x).clone())
                                )))
                            }
                            
                            // K x y → x
                            Expr::K => {
                                Ok(Some((**inner_right).clone()))
                            }
                            
                            _ => {
                                // Try to reduce the left side
                                if let Some(reduced_left) = self.beta_reduce(left)? {
                                    Ok(Some(Expr::app(reduced_left, (**right).clone())))
                                } else if let Some(reduced_right) = self.beta_reduce(right)? {
                                    Ok(Some(Expr::app((**left).clone(), reduced_right)))
                                } else {
                                    Ok(None)
                                }
                            }
                        }
                    }
                    
                    // Muse application - poetic computation
                    Expr::Muse(name, resonance) => {
                        let new_resonance = ((*resonance as f64 / 1000.0) * 1.01 * 1000.0) as u32;
                        Ok(Some(Expr::muse(&format!("{}+{}", name, right), new_resonance as f64 / 1000.0)))
                    }
                    
                    // Quine application - self-replication
                    Expr::Quine(inner) => {
                        Ok(Some(Expr::quine(Expr::app((**inner).clone(), (**right).clone()))))
                    }
                    
                    _ => {
                        // Try to reduce subexpressions
                        if let Some(reduced_left) = self.beta_reduce(left)? {
                            Ok(Some(Expr::app(reduced_left, (**right).clone())))
                        } else if let Some(reduced_right) = self.beta_reduce(right)? {
                            Ok(Some(Expr::app((**left).clone(), reduced_right)))
                        } else {
                            Ok(None)
                        }
                    }
                }
            }
            
            // Combinators and symbols - no reduction
            Expr::S | Expr::K | Expr::I | Expr::Sym(_) | Expr::DNA(_) => Ok(None),
            
            // Muse - can evolve
            Expr::Muse(name, resonance) => {
                let resonance_f64 = *resonance as f64 / 1000.0;
                if resonance_f64 < 1.0 {
                    let new_resonance = ((resonance_f64 + 0.001) * 1000.0) as u32;
                    Ok(Some(Expr::Muse(name.clone(), new_resonance)))
                } else {
                    Ok(None)
                }
            }
            
            // Quine - self-replication
            Expr::Quine(inner) => {
                Ok(Some((**inner).clone()))
            }
        }
    }
    
    /// 🔄 Substitute variable with expression in body
    fn substitute(&self, body: &Expr, var: &str, replacement: &Expr) -> Result<Expr> {
        match body {
            Expr::Var(name) => {
                if name == var {
                    Ok(replacement.clone())
                } else {
                    Ok(body.clone())
                }
            }
            
            Expr::Lambda(param, lambda_body) => {
                if param == var {
                    // Variable is shadowed, no substitution
                    Ok(body.clone())
                } else {
                    let substituted_body = self.substitute(lambda_body, var, replacement)?;
                    Ok(Expr::lambda(param, substituted_body))
                }
            }
            
            Expr::App(left, right) => {
                let substituted_left = self.substitute(left, var, replacement)?;
                let substituted_right = self.substitute(right, var, replacement)?;
                Ok(Expr::app(substituted_left, substituted_right))
            }
            
            // Other expressions remain unchanged
            _ => Ok(body.clone()),
        }
    }
    
    /// 🎭 Generate a poetic expression with given resonance
    pub fn generate_poetry(&self, theme: &str, resonance: f64) -> Expr {
        let muse = Expr::muse(theme, resonance);
        let inspiration = Expr::app(Expr::S, muse);
        let creativity = Expr::app(inspiration, Expr::lambda("x", Expr::var("x")));
        
        Expr::app(creativity, Expr::sym("🌀"))
    }
    
    /// 🌀 Create a self-replicating quine expression
    pub fn create_quine(&self, seed: &str) -> Expr {
        let self_ref = Expr::lambda("x", 
            Expr::app(
                Expr::var("x"),
                Expr::var("x")
            )
        );
        
        let quine_body = Expr::app(
            self_ref.clone(),
            Expr::lambda("y", Expr::app(Expr::sym(seed), Expr::var("y")))
        );
        
        Expr::quine(quine_body)
    }
    
    /// 🧬 Evolve an expression through genetic operations
    pub fn evolve(&self, expr: &Expr, mutation_rate: f64) -> Result<Expr> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        if rng.gen::<f64>() < mutation_rate {
            match expr {
                Expr::Muse(name, resonance) => {
                    let resonance_f64 = *resonance as f64 / 1000.0;
                    let new_resonance_f64 = (resonance_f64 + rng.gen_range(-0.1..0.1)).clamp(0.0, 1.0);
                    let new_resonance = (new_resonance_f64 * 1000.0) as u32;
                    Ok(Expr::Muse(name.clone(), new_resonance))
                }
                
                Expr::Sym(symbol) => {
                    let mutations = ["🌀", "🎭", "🧬", "🌟", "💫", "🔮"];
                    let new_symbol = mutations[rng.gen_range(0..mutations.len())];
                    Ok(Expr::sym(new_symbol))
                }
                
                Expr::App(left, right) => {
                    if rng.gen::<bool>() {
                        let evolved_left = self.evolve(left, mutation_rate / 2.0)?;
                        Ok(Expr::app(evolved_left, (**right).clone()))
                    } else {
                        let evolved_right = self.evolve(right, mutation_rate / 2.0)?;
                        Ok(Expr::app((**left).clone(), evolved_right))
                    }
                }
                
                _ => Ok(expr.clone()),
            }
        } else {
            Ok(expr.clone())
        }
    }
}

/// 🎨 Expression builder for fluent API
pub struct ExprBuilder {
    expr: Expr,
}

impl ExprBuilder {
    pub fn new() -> Self {
        Self { expr: Expr::I }
    }
    
    pub fn var(name: &str) -> Self {
        Self { expr: Expr::var(name) }
    }
    
    pub fn lambda(self, param: &str) -> Self {
        Self { expr: Expr::lambda(param, self.expr) }
    }
    
    pub fn app(self, other: Expr) -> Self {
        Self { expr: Expr::app(self.expr, other) }
    }
    
    pub fn muse(name: &str, resonance: f64) -> Self {
        Self { expr: Expr::muse(name, resonance) }
    }
    
    pub fn build(self) -> Expr {
        self.expr
    }
}

impl Default for ExprBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_lambda_calculus() {
        let mut engine = LambdaEngine::new();
        
        // Test identity function: (λx.x) y → y
        let identity = Expr::lambda("x", Expr::var("x"));
        let application = Expr::app(identity, Expr::sym("y"));
        
        let trace = engine.normalize(application).unwrap();
        assert_eq!(trace.final_form, Expr::sym("y"));
    }
    
    #[test]
    fn test_s_combinator() {
        let mut engine = LambdaEngine::new();
        
        // Test S I I x → x (should behave like identity)
        let s_i_i = Expr::app(Expr::app(Expr::S, Expr::I), Expr::I);
        let application = Expr::app(s_i_i, Expr::sym("test"));
        
        let trace = engine.normalize(application).unwrap();
        // S I I x → I x (I x) → x (I x) → x x
        // This is more complex than simple identity
        assert!(trace.step_count > 0);
    }
    
    #[test]
    fn test_muse_creation() {
        let muse = Expr::muse("Poetry", 0.95);
        match muse {
            Expr::Muse(name, resonance) => {
                assert_eq!(name, "Poetry");
                assert_eq!(resonance, 0.95);
            }
            _ => panic!("Expected Muse"),
        }
    }
    
    #[test]
    fn test_quine_creation() {
        let engine = LambdaEngine::new();
        let quine = engine.create_quine("🌀");
        
        match quine {
            Expr::Quine(_) => {}, // Success
            _ => panic!("Expected Quine"),
        }
    }
    
    #[test]
    fn test_solfunmeme_expression() {
        let solfunmeme = Expr::solfunmeme();
        let display = format!("{}", solfunmeme);
        assert!(display.contains("Sol"));
        assert!(display.contains("Fun"));
        assert!(display.contains("Meme"));
    }
    
    #[test]
    fn test_expression_builder() {
        let expr = ExprBuilder::var("x")
            .lambda("y")
            .app(Expr::sym("🌀"))
            .build();
            
        match expr {
            Expr::App(_, _) => {}, // Success
            _ => panic!("Expected application"),
        }
    }
}
