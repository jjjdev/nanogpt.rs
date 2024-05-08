
pub struct HyperParams {
    pub batch_size: u16,
    pub block_size: u16,
    pub n_embd: u16,
    pub n_head: u16,
    pub n_layer: u16,
    pub n_vocab: u16,
    pub max_iters: u16,
    pub eval_interval: u16,
    pub learning_rate: f32,
    pub eval_iters: u16,
    pub dropout: f32,
}
impl Default for HyperParams {
    fn default() -> Self {
        Self {
            batch_size: 64,
            block_size: 256,
            max_iters: 1000,
            eval_interval: 100,
            learning_rate: 0.00002,
            eval_iters: 10,
            n_embd: 384,
            n_head: 6,
            n_layer: 6,
            n_vocab: 50257,
            dropout: 0.1,
        }
    }
}

/*impl HyperParams {
    pub fn new() -> HyperParams {
        HyperParams {
            batch_size: 64,
            block_size: 256,
            n_embd: 768,
            n_head: 12,
            n_layer: 12,
            n_vocab: 50257,
            max_iters: 1000,
            eval_interval: 100,
            learning_rate: 0.00002,
            eval_iters: 10,
            dropout: 0.1,
        }
    }
}*/