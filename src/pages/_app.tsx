import type { AppProps } from 'next/app';
import { createGlobalStyle } from 'styled-components';
import { ColorModeProvider } from '../context/colourMode';
import useColourMode, { ColourMode } from '../hooks/useColourMode';
import '@blueprintjs/core/lib/css/blueprint.css';
import 'normalize.css/normalize.css';
import '../styles/globals.css';

function MyApp({ Component, pageProps }: AppProps) {
  const colourMode = useColourMode();

  const GlobalStyle = createGlobalStyle<{ colourMode: ColourMode }>`
    ${({ colourMode }) => {
      if (colourMode === ColourMode.Dark) {
        return `
          html {
            color-scheme: dark;
          }
          body {
            color: white;
            background: black;
          }
        `;
      }
    }}
  `;

  return (
    <div className={colourMode === ColourMode.Dark ? 'bp4-dark' : 'bp4-light'}>
      <ColorModeProvider value={colourMode}>
        <GlobalStyle colourMode={colourMode} />
        <Component {...pageProps} />
      </ColorModeProvider>
    </div>
  );
}

export default MyApp;
