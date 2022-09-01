import type { AppProps } from 'next/app';
import '@blueprintjs/core/lib/css/blueprint.css';
import 'normalize.css/normalize.css';
import { createGlobalStyle } from 'styled-components';
import { ColorModeProvider } from '../context/colourMode';
import useColourMode, { ColourMode } from '../hooks/useColourMode';
import '../styles/globals.css';
import { useMemo } from 'react';

function MyApp({ Component, pageProps }: AppProps) {
  const colourMode = useColourMode();

  const GlobalStyle = useMemo(() => {
    if (colourMode === ColourMode.Dark) {
      return createGlobalStyle`
        html {
          color-scheme: dark;
        }
        body {
          color: white;
          background: black;
        }
      `;
    } else {
      return createGlobalStyle``;
    }
  }, [colourMode]);

  return (
    <ColorModeProvider value={colourMode}>
      <GlobalStyle />
      <Component {...pageProps} />
    </ColorModeProvider>
  );
}

export default MyApp;
