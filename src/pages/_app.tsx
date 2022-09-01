import type { AppProps } from 'next/app';
import '@blueprintjs/core/lib/css/blueprint.css';
import 'normalize.css/normalize.css';
import { ColorModeProvider } from '../context/colourMode';
import useColourMode from '../hooks/useColourMode';
import '../styles/globals.css';

function MyApp({ Component, pageProps }: AppProps) {
  const colourMode = useColourMode();

  return (
    <ColorModeProvider value={colourMode}>
      <Component {...pageProps} />
    </ColorModeProvider>
  );
}

export default MyApp;
