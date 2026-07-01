import React from 'react';

interface HeaderProps {
  title: string;
}

const Header: React.FC<HeaderProps> = ({ title }) => {
  var oldVar = 'bad';

  return (
    <header>
      <h1>{title}</h1>
      <nav>
        <a href="/home">Home</a>
        <a href="/about">About</a>
      </nav>
    </header>
  );
};

export default Header;
