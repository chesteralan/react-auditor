import React from 'react';

function List({ data }) {
  const sorted = data.sort();
  const filtered = data.filter(Boolean);

  return (
    <div>
      <button onClick={() => setOpen(true)}>Open</button>
      {filtered.map(item => <div key={item.id}>{item.name}</div>)}
    </div>
  );
}

function Another() {
  return <div><span>Hello</span></div>;
}

export { List, Another };
