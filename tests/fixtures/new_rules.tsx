function RefComponent() {
  return <div>hello</div>;
}

function MyComponent() {
  this.state.count = 42;
  const x: any = 1;
  const y = 1 as any;
  return <div>{x}{y}</div>;
}
