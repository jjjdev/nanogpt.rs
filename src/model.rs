use candle_core::{Device, Result, Tensor};

impl NanoGptModelConfig {
    pub fn init(&self) -> NanoGptModel {
        let mut model = NanoGptModel {
            wte: Tensor::randn(&[self.n_vocab as usize, self.n_embd as usize], &Device::Cpu),
            wpe: Tensor::randn(&[self.block_size as usize, self.n_embd as usize], &Device::Cpu),
            drop: self.dropout,
            h: (0..self.n_layer).map(|_| {
                NanoGptLayer {
                    attn: NanoAttention {
                        c_attn: Tensor::randn(&[self.n_embd as usize, 3 * self.n_embd as usize], &Device::Cpu),
                        c_proj: Tensor::randn(&[self.n_embd as usize, self.n_embd as usize], &Device::Cpu),
                        dropout: self.dropout,
                    },
                    ln_1: NanoLayerNorm {
                        weight: Tensor::randn(&[self.n_embd as usize], &Device::Cpu),
                        bias: Tensor::randn(&[self.n_embd as usize], &Device::Cpu),
                    },
                    mlp: NanoMLP {
                        c_fc: Tensor::randn(&[self.n_embd as usize, 4 * self.n_embd as usize], &Device::Cpu),
                        c_proj: Tensor::randn(&[4 * self.n_embd as usize, self.n_embd as usize], &Device::Cpu),
                        dropout: self.dropout,
                    },
                    ln_2: NanoLayerNorm {
                        weight: Tensor::randn(&[self.n_embd as usize], &Device::Cpu),
                        bias: Tensor::randn(&[self.n_embd as usize], &Device::Cpu),
                    },
                }
            }).collect(),
            output: Tensor::randn(&[self.n_vocab as usize, self.n_embd as usize], &Device::Cpu),
        };
        model
    }
}

