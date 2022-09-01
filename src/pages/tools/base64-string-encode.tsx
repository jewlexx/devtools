import { useEffect, useState } from 'react';
import type { NextPage } from 'next';
import { InputGroup } from '@blueprintjs/core';
import Layout from '../../components/Layout';
import useTauri from '../../hooks/useTauri';

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
    <Layout>
      <InputGroup
        placeholder="Enter the string to be encoded in base64"
        value={input}
        onChange={(e) => setInput(e.target.value)}
      />
      <p>{output}</p>
    </Layout>
  );
};

export const title = 'Base64 String Encode';

export const description = 'Encode a string in base64';

export default Base64StringEncode;
