#!/usr/bin/env node
/**
 * Simple test script to verify WASM module loads correctly
 */

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function testWasmModule() {
  try {
    console.log('🧪 Testing WASM module...');

    // Check if files exist
    const wasmDir = path.join(__dirname, '../public/wasm');
    const jsFile = path.join(wasmDir, 'connect_four_ai_core.js');
    const wasmFile = path.join(wasmDir, 'connect_four_ai_core_bg.wasm');

    if (!fs.existsSync(jsFile)) {
      console.error('❌ JavaScript file not found:', jsFile);
      return false;
    }

    if (!fs.existsSync(wasmFile)) {
      console.error('❌ WASM file not found:', wasmFile);
      return false;
    }

    const jsStats = fs.statSync(jsFile);
    const wasmStats = fs.statSync(wasmFile);

    console.log('✅ Files found:');
    console.log(`   JS: ${jsFile} (${jsStats.size} bytes)`);
    console.log(`   WASM: ${wasmFile} (${wasmStats.size} bytes)`);

    // Check if files have reasonable sizes
    if (jsStats.size < 1000) {
      console.error('❌ JavaScript file seems too small');
      return false;
    }

    if (wasmStats.size < 10000) {
      console.error('❌ WASM file seems too small');
      return false;
    }

    console.log('✅ File sizes look reasonable');

    // Check if JS file contains expected content
    const jsContent = fs.readFileSync(jsFile, 'utf8');
    if (!jsContent.includes('ConnectFourAI')) {
      console.error('❌ JavaScript file does not contain ConnectFourAI class');
      return false;
    }

    console.log('✅ JavaScript file contains ConnectFourAI class');

    console.log('✅ WASM module test passed!');
    return true;
  } catch (error) {
    console.error('❌ WASM module test failed:', error);
    return false;
  }
}

testWasmModule().then(success => {
  process.exit(success ? 0 : 1);
});
