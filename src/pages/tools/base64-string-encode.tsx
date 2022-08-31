import { useEffect, useState } from 'react';
import type { NextPage } from 'next';
import Layout from '../../components/Layout';
import useTauri from '../../lib/usetauri';

const Base64StringEncode: NextPage = () => {
  const [input, setInput] = useState('');
  const [output, setOutput] = useState('');
  const tauri = useTauri();

  useEffect(() => {
    if (tauri !== null) {
      tauri.invoke('base64_parse', { input, encode: true }).then((res) => {
        setOutput(res as string);
      });
    }
  }, [input, tauri]);

  return (
    <div>
      <input value={input} onChange={(e) => setInput(e.target.value)}></input>
      <p>{output}</p>
    </div>
  );
};

export default Base64StringEncode;
