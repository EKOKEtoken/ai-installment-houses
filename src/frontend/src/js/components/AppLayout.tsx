import * as React from 'react';

import Container from './reusable/Container';
import Header from './Header';
import Footer from './Footer';
import AppSuccess from './App/AppSuccess';
import AppError from './App/AppError';

interface Props {
  children: React.ReactNode;
}

const AppLayout = ({ children }: Props) => (
  <Container.Container>
    <AppError />
    <AppSuccess />
    <Header />
    <Container.Container>{children}</Container.Container>
    <Footer />
  </Container.Container>
);

export default AppLayout;
