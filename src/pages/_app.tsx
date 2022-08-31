import type { AppProps } from 'next/app';
import '@blueprintjs/core/lib/css/blueprint.css';
import 'normalize.css/normalize.css';
import '../styles/globals.css';

function MyApp({ Component, pageProps }: AppProps) {
  return <Component {...pageProps} />;
}

export default MyApp;
