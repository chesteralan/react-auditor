import { useState, useEffect } from 'react';

function useAuth() {
  const [user, setUser] = useState(null);

  useEffect(() => {
    fetch('/api/auth/user')
      .then(res => res.json())
      .then(data => setUser(data));
  }, []);

  return user;
}

export default useAuth;
