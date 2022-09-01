import type { GetStaticProps, NextPage } from 'next';
import Link from 'next/link';
import Head from 'next/head';
import Image from 'next/image';
import { useState } from 'react';
import styles from '../styles/Home.module.scss';
import { Card, Elevation } from '@blueprintjs/core';

const Home: NextPage<Props> = ({ tools }) => {
  const [isHovered, setIsHovered] = useState(-1);

  return (
    <div className={styles.container}>
      <Head>
        <title>Create Next App</title>
        <meta name="description" content="Generated by create next app" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
        {tools.map((tool, i) => (
          <Link href={tool.url} key={JSON.stringify({ ...tool, i })}>
            <Card
              elevation={isHovered === i ? Elevation.FOUR : Elevation.ZERO}
              onMouseOver={() => setIsHovered(i)}
              onMouseOut={() => setIsHovered(-1)}
              className={styles.pageLink}
            >
              Base64 String Encode
            </Card>
          </Link>
        ))}
      </main>
    </div>
  );
};

interface Tool {
  title: string;
  description: string;
  url: string;
}

interface Props {
  tools: Tool[];
}

export const getStaticProps: GetStaticProps<Props> = async (context) => {
  const fs = await import('fs/promises');
  const path = await import('path');

  const toolsPath = process.cwd() + '/src/pages/tools';

  const toolsDir = await fs.readdir(toolsPath);

  const toolsImports: Tool[] = await Promise.all(
    toolsDir.map(async (p) => {
      const tool = (await import('./tools/' + p)) as Tool;

      return {
        title: tool.title,
        description: tool.description,
        url: tool.url,
      };
    }),
  );

  toolsImports.forEach((t) => console.log(t.url));

  return {
    props: { tools: toolsImports },
  };
};

export default Home;
