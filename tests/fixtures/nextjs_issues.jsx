import Head from 'next/head';
import Script from 'next/script';

export default function Home() {
  return (
    <div>
      <head>
        <title>My Page</title>
        <script src="https://example.com/widget.js"></script>
      </head>
      <img src="/logo.png" alt="Logo" />
      <a href="/about">About</a>
      <a href="https://example.com">External</a>
      <a href="./contact">Contact</a>
      <Script src="https://example.com/analytics.js" />
    </div>
  );
}
