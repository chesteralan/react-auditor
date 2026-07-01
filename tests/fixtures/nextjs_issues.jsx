import Head from 'next/head';

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
    </div>
  );
}
