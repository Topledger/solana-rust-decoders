import fs from 'fs';
import path from 'path';
import pkg from './pkg/system_program_transfers.js';
const { default: init, parse } = pkg;

async function testSystemFormat() {
  try {
    const wasmPath = path.join(process.cwd(), 'pkg/system_program_transfers_bg.wasm');
    const wasmBytes = fs.readFileSync(wasmPath);
    await init(wasmBytes);
    
    console.log('=== SYSTEM PROGRAM TRANSFERS FORMAT ===\n');
    
    // Test with a Transfer instruction (discriminator 2)
    // [2,0,0,0] + [100,0,0,0,0,0,0,0] = transfer 100 lamports
    const transferData = 'CAAAAAAIAAAAAAAAAA=='; // base64 for test
    
    try {
      // Convert to base58 first
      const result = parse('AgAAAABkAAAAAAAA', []); // Simple test
      console.log('System Program Result:', JSON.stringify(result, null, 2));
      
      console.log('\nStructure:');
      console.log('- Has instruction_type:', 'instruction_type' in result);
      console.log('- Has args:', 'args' in result);
      console.log('- Has accounts:', 'accounts' in result);
      console.log('- typeof args:', typeof result.args);
      console.log('- typeof accounts:', typeof result.accounts);
      
    } catch (e) {
      console.log('Parse error:', e.message);
    }
    
  } catch (error) {
    console.error('Init error:', error.message);
  }
}

testSystemFormat();
