import type * as TauriApi from '@tauri-apps/api';
import { useState, useEffect } from 'react';

const useTauri = () => {
  const [tauriApi, setTauriApi] = useState<typeof TauriApi | null>(null);

  useEffect(() => {
    import('@tauri-apps/api').then((api) => {
      setTauriApi(api);
    });
  });

  return tauriApi;
};

export default useTauri;
