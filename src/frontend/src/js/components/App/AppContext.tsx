import * as React from 'react';
import { WalletProvider } from 'react-ic-wallet';
import { getUserIcWallet } from '../../storage';

interface Context {
  appSuccess?: string;
  appError?: string;
  icWallet?: WalletProvider;
  setAppError: (error?: string) => void;
  setAppSuccess: (message?: string) => void;
  setIcWallet?: (icWallet: WalletProvider | undefined) => void;
}

const AppContext = React.createContext<Context>({
  setAppError: () => {},
  setAppSuccess: () => {},
});

const AppContextProvider = ({ children }: { children?: React.ReactNode }) => {
  const [icWallet, setIcWallet] = React.useState<WalletProvider | undefined>();
  const [appError, setAppError] = React.useState<string | undefined>();
  const [appSuccess, setAppSuccess] = React.useState<string>();

  React.useEffect(() => {
    const userWallet = getUserIcWallet();
    if (userWallet) {
      setIcWallet(userWallet);
    }
  }, []);

  return (
    <AppContext.Provider
      value={{
        appError,
        appSuccess,
        icWallet,
        setAppError,
        setAppSuccess,
        setIcWallet,
      }}
    >
      {children}
    </AppContext.Provider>
  );
};

export default AppContextProvider;

export const useAppContext = () => React.useContext(AppContext);
