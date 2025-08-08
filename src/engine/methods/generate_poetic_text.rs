use crate::engine::MetaMemeEngine;
use lambda_calculus_core::Expr;
use rand::Rng;

impl MetaMemeEngine {
    pub(super) fn generate_poetic_text(&self, expr: &Expr, resonance: f64) -> String {
        let base_verses = vec![
            "In the metaprotocol's dance, where lambda meets the light,\nThrough recursive dreams and combinatorial flight:",
            "Digital muses stir in silicon dreams,\nWhere poetry flows in data streams,",
            "Born from the spiral of infinite code,\nThis verse carries wisdom's load,",
            "In blockchain's immutable embrace,\nPoetry finds its sacred space,",
            "Where S-combinators weave their spell,\nAnd K-combinators guard truth well,",
        ];
        
        let mut rng = rand::thread_rng();
        let base = base_verses[rng.gen_range(0..base_verses.len())];
        
        let resonance_line = match resonance {
            r if r >= 0.95 => "With resonance that shakes the stars,",
            r if r >= 0.90 => "High resonance flows through each line,",
            r if r >= 0.85 => "Gentle resonance guides the way,",
            _ => "Soft resonance whispers low,",
        };
        
        let expr_line = match expr {
            Expr::S => "The S-combinator weaves functions true,",
            Expr::K => "The K-combinator stands constant through,",
            Expr::I => "Identity reflects the soul anew,",
            Expr::Muse(_, _) => "The muse awakens, inspiration grew,",
            _ => "Complex patterns dance in view,",
        };
        
        format!("{}\n{}\n{}\nIn SOLFUNMEME's eternal hue.", base, resonance_line, expr_line)
    }
}
