import type { FunctionComponent, ReactNode } from 'react';
import styles from './index.module.scss';

const Layout: FunctionComponent<{
  children: ReactNode;
}> = ({ children }) => {
  return <div className={styles.container}>{children}</div>;
};

export default Layout;
