import React from 'react';
import moment from 'moment';

interface User {
  id: number;
  name: string;
  email: string;
}

interface UserListProps {
  users: User[];
}

const UserList: React.FC<UserListProps> = ({ users }) => {
  return (
    <div>
      {users.map((user) => (
        <div key={user.id}>
          <strong>{user.name}</strong>
          <span>{user.email}</span>
        </div>
      ))}
    </div>
  );
};

export default UserList;
