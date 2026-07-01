import React from 'react';

var unusedVar = 42;

function MyComponent() {
  console.log('rendering');

  var items = [1, 2, 3];
  return (
    <div style={{ color: 'red' }}>
      {items.map(function(item, index) {
        return <div key={index}>{item}</div>;
      })}
      <button onClick={function() {}}>Click</button>
    </div>
  );
}

export default MyComponent;
