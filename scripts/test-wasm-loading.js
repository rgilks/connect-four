#!/usr/bin/env node
/**
 * Test script to verify WASM module loading in a browser-like environment
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function testWasmLoading() {
  try {
    console.log('🧪 Testing WASM module loading...');

    // Check if the development server is running
    const response = await fetch('http://localhost:3000/wasm/connect_four_ai_core.js');
    if (!response.ok) {
      console.error('❌ Development server not running or WASM files not accessible');
      console.error('   Please run: npm run dev');
      return false;
    }

    console.log('✅ Development server is running');

    // Check if WASM file is accessible
    const wasmResponse = await fetch('http://localhost:3000/wasm/connect_four_ai_core_bg.wasm');
    if (!wasmResponse.ok) {
      console.error('❌ WASM file not accessible');
      return false;
    }

    console.log('✅ WASM file is accessible');

    // Check if ML weights are accessible
    const weightsResponse = await fetch(
      'http://localhost:3000/ml/data/weights/ml_ai_weights_best.json'
    );
    if (!weightsResponse.ok) {
      console.error('❌ ML weights not accessible');
      return false;
    }

    console.log('✅ ML weights are accessible');

    // Check if genetic parameters are accessible
    const paramsResponse = await fetch('http://localhost:3000/ml/data/genetic_params/evolved.json');
    if (!paramsResponse.ok) {
      console.error('❌ Genetic parameters not accessible');
      return false;
    }

    console.log('✅ Genetic parameters are accessible');

    console.log('✅ All WASM assets are accessible!');
    console.log('   You can now test the application in your browser');
    return true;
  } catch (error) {
    console.error('❌ WASM loading test failed:', error.message);
    console.error('   Make sure the development server is running: npm run dev');
    return false;
  }
}

testWasmLoading().then(success => {
  process.exit(success ? 0 : 1);
});
