import { useEffect, useState } from 'react';
import useTauri from '../../lib/usetauri';
import type { NextPage } from 'next';

const Base64StringEncode: NextPage = () => {
  const [input, setInput] = useState('');
  const [output, setOutput] = useState('');
  const tauri = useTauri();

  useEffect(() => {
    import('@tauri-apps/api').then(({ invoke }) => {
      invoke('base64_parse', {
        input,
        encode: true,
      }).then((result) => {
        setOutput(result as string);
      });
    });
  }, [input]);

  return (
    <>
      <input value={input} onChange={(e) => setInput(e.target.value)}></input>
      <p>{output}</p>
    </>
  );
};

export default Base64StringEncode;
