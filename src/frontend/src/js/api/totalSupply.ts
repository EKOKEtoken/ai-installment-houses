import { ledgerJsonRequest } from './api';

const totalSupply = async (): Promise<number> => {
  const supply = await ledgerJsonRequest('totalSupply', {
    totalSupply: 1024,
  });

  return supply.totalSupply;
};

export default totalSupply;
