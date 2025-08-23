CREATE OR REPLACE FUNCTION `tt-solana.tl.bpf_decoder`(
  raw_data ARRAY<STRUCT<
    join_key STRING,
    data STRING,
    account_arguments ARRAY<STRING>
  >>
)
RETURNS ARRAY<JSON>
LANGUAGE js
OPTIONS (
  library = [
    'gs://tt-bq-js/lib/inexorabletash.encoding.js',
    'gs://tt-bq-js/solana/BPF_loader/bpf_decoder.js',
    'gs://tt-bq-js/solana/BPF_loader/wasm_bytes.js',
    'gs://tt-bq-js/lib/inflate.min.js'
  ]
)
AS """
const memory = new WebAssembly.Memory({ initial: 256, maximum: 256 });

const env = {
  abortStackOverflow: _ => { throw new Error('overflow'); },
  table: new WebAssembly.Table({ initial: 0, maximum: 0, element: 'anyfunc' }),
  tableBase: 0,
  memory: memory,
  memoryBase: 1024,
  STACKTOP: 0,
  STACK_MAX: memory.buffer.byteLength
};

const imports = { env };

async function run() {
  try {
    // 1) Inflate and instantiate your WASM module
    const inflate = new Zlib.Inflate(bytes);
    const plain = inflate.decompress();
    await wasm_bindgen(plain);

    // Verify WASM module is properly initialized
    if (typeof parse !== 'function') {
      throw new Error('WASM module not properly initialized - parse function not available');
    }

    // 2) Process every row, catching any errors
    return raw_data.map(d => {
      // base output
      const out = {
        join_key:          d.join_key,
        raw_data:          d.data,
        account_arguments: d.account_arguments,
        error:             null
      };

      try {
        const parsed = parse(d.data, d.account_arguments);

        // stringify & parse to handle BigInt → string, etc.
        const safe = JSON.parse(JSON.stringify(parsed, (k,v) =>
          typeof v === 'bigint' ? v.toString() : v
        ));

        // Validate expected structure
        if (typeof safe.instruction_type !== 'string') {
          throw new Error('Missing or invalid instruction_type');
        }
        
        // For BPF Loader, args should be an object (can be empty for some instructions)
        if (safe.args != null && typeof safe.args !== 'object') {
          throw new Error('Expected args to be an object or null');
        }

        // Convert the args-object into an array of {name, value}
        // Handle case where args might be null/undefined for some BPF instructions
        if (safe.args && typeof safe.args === 'object' && !Array.isArray(safe.args)) {
          out.args = Object.entries(safe.args).map(([name, value]) => ({
            name,
            value
          }));
        } else {
          out.args = []; // Empty args for instructions that don't have arguments
        }

        // Copy instruction_type and input_accounts
        out.instruction_type = safe.instruction_type;
        out.input_accounts   = safe.input_accounts || [];

      } catch (e) {
        out.error = e.message || String(e);
        // Clean up output on error
        delete out.instruction_type;
        delete out.args;
        delete out.input_accounts;
      }

      return out;
    });

  } catch (e) {
    // Return error for all rows if WASM initialization fails
    return raw_data.map(d => ({
      join_key: d.join_key,
      raw_data: d.data,
      account_arguments: d.account_arguments,
      error: `WASM initialization failed: ${e.message || String(e)}`
    }));
  }
}

return run();
"""; 