import type { ReactNode, FunctionComponent as FCReact } from 'react';

declare module 'react' {
  interface FunctionComponent<
    P = {
      children: ReactNode;
    },
  > extends FCReact<P> {}
}
