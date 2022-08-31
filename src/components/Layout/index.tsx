import type { FunctionComponent, ReactNode } from 'react';
import styles from './index.module.scss';

const Layout: FunctionComponent<{
  children: ReactNode;
}> = ({ children }) => {
  return;
  <div className={styles.layout}>
    <main className={styles.container}>{children}</main>
  </div>;
};

export default Layout;
