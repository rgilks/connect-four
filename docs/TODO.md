# TODO

## ✅ COMPLETED: WASM AI Integration

**Status**: The sophisticated Rust/WASM AI system has been successfully integrated and is now being used in the game.

### Completed Actions

- [x] **Create WASM integration service** (`src/lib/wasm-ai-service.ts`)
- [x] **Import WASM module** in frontend application
- [x] **Replace JavaScript AI** with WASM Classic AI in `game-logic.ts`
- [x] **Test WASM integration** and performance
- [x] **Update game store** to use WASM AI instead of JavaScript AI
- [x] **Add error handling** for WASM loading failures
- [x] **Verify WASM builds** are working correctly
- [x] **Fix player value conversion issues** (consistent lowercase format)
- [x] **Add comprehensive test coverage** for WASM AI service
- [x] **Update documentation** to reflect current state

## High Priority

- [ ] Improve ML AI performance vs Classic AI
- [ ] Investigate ONNX and 'trace' for ML AI
- [ ] Optimize neural network architecture

## Medium Priority

- [ ] Implement GPU training acceleration with Rust
  - Consider frameworks like Burn, tch-rs, or custom CUDA/Metal implementation
  - Focus on Apple Silicon Metal backend for optimal performance
  - Maintain compatibility with existing CPU training pipeline
- [ ] Add self-play reinforcement learning
- [ ] Implement Monte Carlo Tree Search on top of neural network
- [ ] Optimize feature engineering (review 150 features)

## Low Priority

- [ ] Add multiplayer support
- [ ] Create mobile app version
