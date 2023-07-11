#[derive(Debug, Clone, Copy)]
pub struct Rules {
    pub insertion: f32,
    pub deletion: f32,
    pub substitution: f32,
    pub transposition: f32,
}