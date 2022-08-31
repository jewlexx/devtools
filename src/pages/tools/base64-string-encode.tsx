import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api';
import type { NextPage } from 'next';

const Base64StringEncode: NextPage = () => {
  const [input, setInput] = useState('');
  const [output, setOutput] = useState('');

  useEffect(() => {
    invoke('base64_parse', {
      input,
      encode: true,
    }).then((result) => {
      setOutput(result as string);
    });
  }, [input]);

  return (
    <>
      <p>{output}</p>
    </>
  );
};
