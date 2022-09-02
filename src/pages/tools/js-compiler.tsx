import type { NextPage } from 'next';
import initSwc, { transformSync } from '@swc/wasm-web';

const JsCompiler: NextPage = () => {
  return <></>;
};

export const title = 'Js Compiler';
export const description = 'Compile javascript';
export const url = '/tools/js-compiler';

export default JsCompiler;
