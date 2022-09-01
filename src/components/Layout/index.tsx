import Link from 'next/link';
import type { FunctionComponent, ReactNode } from 'react';
import styles from './index.module.scss';

const Layout: FunctionComponent<{
  children: ReactNode;
}> = ({ children }) => {
  return (
    <>
      <nav className={styles.navigation}>
        <Link href="/">
          <h1 className={styles.title}>The Developer&apos;s Best Friend</h1>
        </Link>
      </nav>
      <div className={styles.navPlaceholder}></div>
      <main className={styles.layout}>
        <main className={styles.container}>{children}</main>
      </main>
    </>
  );
};

export default Layout;
