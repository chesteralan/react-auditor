type User = {
  name: string;
  age: number;
};

interface Empty {}

function process(data: any): string {
  const value = data!;
  return value.name as string;
}

export default process;
