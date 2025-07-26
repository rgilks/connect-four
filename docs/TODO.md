# TODO

## 🚨 CRITICAL: WASM AI Integration

**Current Issue**: The sophisticated Rust/WASM AI system exists but is NOT being used. The game runs on basic JavaScript AI.

### Immediate Actions Required

- [ ] **Create WASM integration service** (`src/lib/wasm-ai-service.ts`)
- [ ] **Import WASM module** in frontend application
- [ ] **Replace JavaScript AI** with WASM Classic AI in `game-logic.ts`
- [ ] **Test WASM integration** and performance
- [ ] **Update game store** to use WASM AI instead of JavaScript AI
- [ ] **Add error handling** for WASM loading failures
- [ ] **Verify WASM builds** are working correctly

### Integration Steps

1. **WASM Service Layer**
   - Create service to handle WASM module loading
   - Implement AI move calculation interface
   - Add fallback to JavaScript AI if WASM fails

2. **Game Logic Replacement**
   - Replace `makeAIMove` function to use WASM
   - Maintain same interface for compatibility
   - Add AI type selection (Classic, ML, Heuristic)

3. **UI Updates**
   - Show AI thinking indicators
   - Display move analysis when available
   - Add AI strength/type selection

4. **Testing & Validation**
   - Verify WASM AI performance vs JavaScript AI
   - Test all AI types work correctly
   - Ensure fallback mechanisms work

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
