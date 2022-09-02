import type { NextPage } from 'next';
import { useEffect, useState } from 'react';
import initSwc, { transformSync } from '@swc/wasm-web';

const JsCompiler: NextPage = () => {
  const [initialized, setInitialized] = useState(false);

  useEffect(() => {
    async function importAndRunSwcOnMount() {
      await initSwc();
      setInitialized(true);
    }
    importAndRunSwcOnMount();
  }, []);

  function compile() {
    if (!initialized) {
      return;
    }
    const result = transformSync(`console.log('hello')`, {});
    console.log(result);
  }

  return (
    <div className="App">
      <button onClick={compile}>Compile</button>
    </div>
  );
};

export const title = 'Js Compiler';
export const description = 'Compile javascript';
export const url = '/tools/js-compiler';

export default JsCompiler;
