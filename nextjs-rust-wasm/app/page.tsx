"use client";

import Image from "next/image";
import styles from "./page.module.css";
import init, { greet, fibonacci } from "../rust-lib/pkg/rust_lib";
import { useEffect, useState } from "react";

export default function Home() {
  const [message, setMessage] = useState("");
  const [fib, setFib] = useState(0);

  useEffect(() => {
    init().then(() => {
      setMessage(greet("Ethan"));
      setFib(fibonacci(10));
    });
  }, []);

  return (
    <main className={styles.main}>
      <div className={styles.description}>
        <p>
          Get started by editing&nbsp;
          <code className={styles.code}>app/page.tsx</code>
        </p>
        <div>
          <a
            href="https://vercel.com?utm_source=create-next-app&utm_medium=appdir-template&utm_campaign=create-next-app"
            target="_blank"
            rel="noopener noreferrer"
          >
            By{" "}
            <Image
              src="/vercel.svg"
              alt="Vercel Logo"
              className={styles.vercelLogo}
              width={100}
              height={24}
              priority
            />
          </a>
        </div>
      </div>
      <div className={styles.center}>
        <h1>{message}</h1>
      </div>
      <div className={styles.center}>
        <p>The 10th Fibonacci number is: {fib}</p>
      </div>
    </main>
  );
}
