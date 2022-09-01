import type { GetStaticProps, NextPage } from 'next';
import Link from 'next/link';
import Head from 'next/head';
import Image from 'next/image';
import { useState } from 'react';
import styles from '../styles/Home.module.css';
import { Card, Elevation } from '@blueprintjs/core';

const Home: NextPage<Props> = ({ tools }) => {
  const [isHovered, setIsHovered] = useState(false);

  return (
    <div className={styles.container}>
      <Head>
        <title>Create Next App</title>
        <meta name="description" content="Generated by create next app" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        <Card
          elevation={isHovered ? Elevation.TWO : Elevation.ZERO}
          onMouseOver={() => setIsHovered(true)}
          onMouseOut={() => setIsHovered(false)}
        >
          <Link href="/tools/base64-string-encode">Base64 String Encode</Link>
        </Card>
      </main>

      <footer className={styles.footer}>
        <a
          href="https://vercel.com?utm_source=create-next-app&utm_medium=default-template&utm_campaign=create-next-app"
          target="_blank"
          rel="noopener noreferrer"
        >
          Powered by{' '}
          <span className={styles.logo}>
            <Image src="/vercel.svg" alt="Vercel Logo" width={72} height={16} />
          </span>
        </a>
      </footer>
    </div>
  );
};

interface Props {
  tools: {
    title: string;
    description: string;
  }[];
}

export const getStaticProps: GetStaticProps<Props> = async (context) => {
  const fs = await import('fs/promises');
  const path = await import('path');
  const filename = __filename;

  const toolsPath = path.join(path.dirname(filename), 'tools');

  const toolsDir = await fs.readdir(toolsPath);

  const toolsImports = await Promise.all(
    toolsDir.map((p) => {
      return import(p) as Promise<Props>;
    }),
  );

  return {
    props: { tools: toolsImports },
  };
};

export default Home;
