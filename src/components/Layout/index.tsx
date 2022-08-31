import type { FunctionComponent, ReactNode } from 'react';
import styles from './index.scss';

const Layout: FunctionComponent<{
  children: ReactNode;
}> = ({ children }) => {
  return <>{children}</>;
};

export default Layout;
