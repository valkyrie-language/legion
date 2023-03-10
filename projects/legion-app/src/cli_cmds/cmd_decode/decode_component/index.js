export function instantiate(getCoreModule, imports, instantiateCore = WebAssembly.instantiate) {
  
  let curResourceBorrows = [];
  
  let dv = new DataView(new ArrayBuffer());
  const dataView = mem => dv.buffer === mem.buffer ? dv : dv = new DataView(mem.buffer);
  
  function getErrorPayload(e) {
    if (e && hasOwnProperty.call(e, 'payload')) return e.payload;
    if (e instanceof Error) throw e;
    return e;
  }
  
  const handleTables = [];
  
  const hasOwnProperty = Object.prototype.hasOwnProperty;
  
  const T_FLAG = 1 << 30;
  
  function rscTableCreateOwn (table, rep) {
    const free = table[0] & ~T_FLAG;
    if (free === 0) {
      table.push(0);
      table.push(rep | T_FLAG);
      return (table.length >> 1) - 1;
    }
    table[0] = table[free << 1];
    table[free << 1] = 0;
    table[(free << 1) + 1] = rep | T_FLAG;
    return free;
  }
  
  function rscTableRemove (table, handle) {
    const scope = table[handle << 1];
    const val = table[(handle << 1) + 1];
    const own = (val & T_FLAG) !== 0;
    const rep = val & ~T_FLAG;
    if (val === 0 || (scope & T_FLAG) !== 0) throw new TypeError('Invalid handle');
    table[handle << 1] = table[0] | T_FLAG;
    table[0] = handle | T_FLAG;
    return { rep, scope, own };
  }
  
  const symbolRscHandle = Symbol('handle');
  
  const symbolRscRep = Symbol.for('cabiRep');
  
  const symbolDispose = Symbol.dispose || Symbol.for('dispose');
  
  
  const module0 = getCoreModule('index.core.wasm');
  const module1 = getCoreModule('index.core2.wasm');
  
  const { getStdout } = imports['@bytecodealliance/preview2-shim/cli/stdout'];
  const { Error: Error$1 } = imports['@bytecodealliance/preview2-shim/io/error'];
  const { OutputStream } = imports['@bytecodealliance/preview2-shim/io/streams'];
  let gen = (function* init () {
    let exports0;
    let memory0;
    let realloc0;
    const handleTable2 = [T_FLAG, 0];
    const captureTable2= new Map();
    let captureCnt2 = 0;
    handleTables[2] = handleTable2;
    const handleTable0 = [T_FLAG, 0];
    const captureTable0= new Map();
    let captureCnt0 = 0;
    handleTables[0] = handleTable0;
    
    function trampoline0(arg0, arg1, arg2, arg3) {
      var handle1 = arg0;
      var rep2 = handleTable2[(handle1 << 1) + 1] & ~T_FLAG;
      var rsc0 = captureTable2.get(rep2);
      if (!rsc0) {
        rsc0 = Object.create(OutputStream.prototype);
        Object.defineProperty(rsc0, symbolRscHandle, { writable: true, value: handle1});
        Object.defineProperty(rsc0, symbolRscRep, { writable: true, value: rep2});
      }
      curResourceBorrows.push(rsc0);
      var ptr3 = arg1;
      var len3 = arg2;
      var result3 = new Uint8Array(memory0.buffer.slice(ptr3, ptr3 + len3 * 1));
      let ret;
      try {
        ret = { tag: 'ok', val: rsc0.blockingWriteAndFlush(result3)};
      } catch (e) {
        ret = { tag: 'err', val: getErrorPayload(e) };
      }
      for (const rsc of curResourceBorrows) {
        rsc[symbolRscHandle] = undefined;
      }
      curResourceBorrows = [];
      var variant6 = ret;
      switch (variant6.tag) {
        case 'ok': {
          const e = variant6.val;
          dataView(memory0).setInt8(arg3 + 0, 0, true);
          break;
        }
        case 'err': {
          const e = variant6.val;
          dataView(memory0).setInt8(arg3 + 0, 1, true);
          var variant5 = e;
          switch (variant5.tag) {
            case 'last-operation-failed': {
              const e = variant5.val;
              dataView(memory0).setInt8(arg3 + 4, 0, true);
              if (!(e instanceof Error$1)) {
                throw new TypeError('Resource error: Not a valid "Error" resource.');
              }
              var handle4 = e[symbolRscHandle];
              if (!handle4) {
                const rep = e[symbolRscRep] || ++captureCnt0;
                captureTable0.set(rep, e);
                handle4 = rscTableCreateOwn(handleTable0, rep);
              }
              dataView(memory0).setInt32(arg3 + 8, handle4, true);
              break;
            }
            case 'closed': {
              dataView(memory0).setInt8(arg3 + 4, 1, true);
              break;
            }
            default: {
              throw new TypeError(`invalid variant tag value \`${JSON.stringify(variant5.tag)}\` (received \`${variant5}\`) specified for \`StreamError\``);
            }
          }
          break;
        }
        default: {
          throw new TypeError('invalid variant specified for result');
        }
      }
    }
    
    function trampoline1() {
      const ret = getStdout();
      if (!(ret instanceof OutputStream)) {
        throw new TypeError('Resource error: Not a valid "OutputStream" resource.');
      }
      var handle0 = ret[symbolRscHandle];
      if (!handle0) {
        const rep = ret[symbolRscRep] || ++captureCnt2;
        captureTable2.set(rep, ret);
        handle0 = rscTableCreateOwn(handleTable2, rep);
      }
      return handle0;
    }
    let exports1;
    Promise.all([module0, module1]).catch(() => {});
    ({ exports: exports0 } = yield instantiateCore(yield module0));
    memory0 = exports0.memory;
    realloc0 = exports0.realloc;
    ({ exports: exports1 } = yield instantiateCore(yield module1, {
      'wasi:cli/stdout@0.2.0': {
        'get-stdout': trampoline1,
      },
      'wasi:io/streams@0.2.0': {
        '[method]output-stream.blocking-write-and-flush': trampoline0,
      },
    }));
    return {};
  })();
  let promise, resolve, reject;
  function runNext (value) {
    try {
      let done;
      do {
        ({ value, done } = gen.next(value));
      } while (!(value instanceof Promise) && !done);
      if (done) {
        if (resolve) return resolve(value);
        else return value;
      }
      if (!promise) promise = new Promise((_resolve, _reject) => (resolve = _resolve, reject = _reject));
      value.then(nextVal => done ? resolve() : runNext(nextVal), reject);
    }
    catch (e) {
      if (reject) reject(e);
      else throw e;
    }
  }
  const maybeSyncReturn = runNext(null);
  return promise || maybeSyncReturn;
}
