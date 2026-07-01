import React, { useState, useEffect, useMemo, useCallback } from 'react';

function notPascalCase() {
  return <div>hello</div>;
}

class MyClass extends React.Component {
  render() {
    return <div>render 1</div>;
  }
  render() {
    return <div>render 2</div>;
  }
}

function MyComponent() {
  const [count, setCount] = useState(0);
  const [items] = useState([1, 2, 3]);

  setTimeout(() => {}, 1000);

  setCount(1);

  const val = useMemo(() => 42, []);
  const cb = useCallback(() => {}, []);

  if (count > 0) {
    useEffect(() => {}, []);
  }

  useEffect(() => {
    setCount(0);
  }, []);

  useEffect(() => {
    setCount(2);
  });

  const list = items.map(item => <Item value={item} />);

  return (
    <div>
      {list}
      <MyClass />
      <div id="x" id="y">duplicate props</div>
    </div>
  );
}

function Item({ value }) {
  return <span>{value}</span>;
}

export default MyComponent;
