# nanoGPT.rs
Rust Implementation of MiniGPT - Training GPT-2 (CPU, fp32) in Rust as lightweight as possible.  We've all seen Andrej Karpathy's famous (or infamous!) minigpt project at https://github.com/karpathy/nanoGPT for python.

Here we're building a simple and fast implementation of miniGPT that will reproduce GPT-2 level using rust.  The aim is to build something that can iterate the training loop fast in as little lines of code as possible.  The code is a very simple foundation for us to hack and improve, learn to train models from scratch, or finetune pretrained checkpoints. 
 Let's go!!

Note: Still learning Rust, so any comments, improvements, or criticisms are welcome!

## Dependencies
This uses the HuggingFace Candle library - a minimalist Rust ML library that is made to look and feel like pytorch!  The instructions and code for setup can be found here: https://github.com/huggingface/candle. 

Note: I initially wanted to use the rust bindings for the C++ API for this [(tch-rs)](https://github.com/LaurentMazare/tch-rs), but it imports the entire pytorch library into the runtime :()


## Links and Resources
- Attention is All You Need paper: https://arxiv.org/abs/1706.03762
- OpenAI GPT-3 paper: https://arxiv.org/abs/2005.14165 

- akarpathy's google colab tutorial - https://colab.research.google.com/drive/1JMLa53HDuA-i7ZBmqV7ZnA3c_fvtXnx-?usp=sharing

- akarpathy's youtube tutorial - https://www.youtube.com/@AndrejKarpathy

## Credit
All credit goes to Andrei Karpathy!!  Thank you for all your contributions toward making ML/DL and LLM knowledge accessible to everyone!!  Please take a look at his incredible learning materials on github (https://github.com/karpathy) and his youtube (link above).

