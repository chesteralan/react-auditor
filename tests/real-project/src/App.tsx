import React, { useState, useEffect } from 'react';
import Header from './components/Header';
import UserList from './components/UserList';

interface AppProps {
  title: string;
}

const App: React.FC<AppProps> = (props) => {
  const [users, setUsers] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    fetch('/api/users')
      .then(res => res.json())
      .then(data => setUsers(data));
  }, []);

  const handleClick = () => {
    console.log('clicked');
  };

  return (
    <div>
      <Header title={props.title} />
      <button onClick={handleClick}>Click me</button>
      {loading && <div>Loading...</div>}
      <UserList users={users} />
    </div>
  );
};

export default App;
