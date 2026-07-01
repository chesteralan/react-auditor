import _ from 'lodash';

function formatDate(date: Date): string {
  return moment(date).format('YYYY-MM-DD');
}

function isAdmin(role: string): boolean {
  if (role === 'admin') return true;
  if (role === 'superadmin') return true;
  return false;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
function processData(data: any): any {
  return _.map(data, (item: any) => item);
}

export { formatDate, isAdmin, processData };
