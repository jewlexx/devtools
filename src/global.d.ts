import type { ReactNode, FunctionComponent as FCReact } from 'react';

declare module 'react' {
  export interface FunctionComponent<
    P = {
      children: ReactNode;
    },
  > {
    (props: P, context?: any): ReactElement<any, any> | null;
    propTypes?: WeakValidationMap<P> | undefined;
    contextTypes?: ValidationMap<any> | undefined;
    defaultProps?: Partial<P> | undefined;
    displayName?: string | undefined;
  }
}
