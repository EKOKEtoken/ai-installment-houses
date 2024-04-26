import * as React from 'react';
import { IcWalletProvider } from 'react-ic-wallet';
import { Route, Routes, useLocation } from 'react-router-dom';

import { getIdFromHash } from './js/utils/routes';
import Footer from './js/components/Footer';
import SeoEngine from './js/components/SeoEngine';
import NotFound from './js/pages/NotFound';

import AppContextProvider, {
  useAppContext,
} from './js/components/App/AppContext';

const App = () => (
  <AppContextProvider>
    <AppLayoutWrapper />
  </AppContextProvider>
);

const AppLayoutWrapper = () => {
  const { icWallet } = useAppContext();
  const { pathname } = useLocation();

  React.useEffect(() => {
    const hash = getIdFromHash();
    if (!hash) {
      window.scrollTo(0, 0);
    } else {
      document.getElementById(hash)?.scrollIntoView({ behavior: 'smooth' });
    }
  }, [pathname]);

  return (
    <IcWalletProvider provider={icWallet}>
      <SeoEngine />
      <main>
        <Routes>
          <Route path="/" element={<> </>} />
          {/* catch all */}
          <Route path="/*" element={<NotFound />} />
        </Routes>
      </main>
      <Footer />
    </IcWalletProvider>
  );
};

export default App;
