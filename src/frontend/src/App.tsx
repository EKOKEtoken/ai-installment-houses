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
import Home from './js/pages/Home';
import Header from './js/components/Header';
import Token from './js/pages/Token';

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
      <Header />
      <main>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/token/:tokenId" element={<Token />} />
          {/* catch all */}
          <Route path="/*" element={<NotFound />} />
        </Routes>
      </main>
      <Footer />
    </IcWalletProvider>
  );
};

export default App;
