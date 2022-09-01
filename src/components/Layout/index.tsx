import type { FunctionComponent, ReactNode } from 'react';
import styled from 'styled-components';
import styles from './index.module.scss';

const Layout: FunctionComponent<{
  children: ReactNode;
}> = ({ children }) => {
  return (
    <>
      <nav className={styles.navigation}>
        <button> HEHEHEHEHE </button>
      </nav>
      <div className={styles.navPlaceholder}></div>
      <div className={styles.layout}>
        <main className={styles.container}>{children}</main>
      </div>
    </>
  );
};

export default Layout;
