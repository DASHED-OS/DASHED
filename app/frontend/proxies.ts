import { Wallet } from './near-wallet';

// Function to create a new wallet instance
export const createWallet = (contractAddress: string) => {
  return new Wallet({ createAccessKeyFor: contractAddress });
};

// Function to call a method on the NEAR contract
export const callContractMethod = async (wallet: Wallet, method: string, args: any, contractId: string) => {
  try {
    return await wallet.callMethod({ method, args, contractId });
  } catch (error) {
    console.error('Error calling contract method:', error);
    throw error;
  }
};

// Function to view a method on the NEAR contract
export const viewContractMethod = async (wallet: Wallet, method: string, contractId: string) => {
  try {
    return await wallet.viewMethod({ method, contractId });
  } catch (error) {
    console.error('Error viewing contract method:', error);
    throw error;
  }
};