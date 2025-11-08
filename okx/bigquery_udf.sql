-- BigQuery UDF for OKX DEX Router v2 Decoder
-- This function decodes OKX DEX instruction data from Solana transactions

CREATE OR REPLACE FUNCTION `tt-solana.tl.okx_dex_router_v2`(
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
    'gs://tt-bq-js/lib/inexorabletask.encoding.js',
    'gs://tt-bq-js/solana/okx/okx.js',
    'gs://tt-bq-js/solana/okx/wasm_bytes.js',
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

const { parse } = wasm_bindgen;

async function run() {
  // 1) Inflate and instantiate your WASM module
  const inflate = new Zlib.Inflate(bytes);
  const plain = inflate.decompress();
  await wasm_bindgen(plain);

  // 2) Process every row, catching any errors
  const result = raw_data.map(d => {
    try {
      // call into your wasm; assume it returns a JSON-able object
      const parsed = parse(d.data, d.account_arguments);
      // if you want join_key always included in the output:
      return Object.assign({ join_key: d.join_key }, parsed);
    } catch (e) {
      // on error, return the original fields plus the error message
      return {
        join_key:         d.join_key,
        data:             d.data,
        account_arguments: d.account_arguments,
        error:            e.message || String(e)
      };
    }
  });

  return result;
}

return run();
""";

