import * as React from 'react';

import Page from '../components/reusable/Page';
import Hero from './Home/Hero';
import ListedTokens from './Home/ListedTokens';
import Container from '../components/reusable/Container';

const Home = () => (
  <Page.BlankPage>
    <Container.FlexCols className="w-full items-center justify-start gap-8">
      <Hero />
      <ListedTokens />
    </Container.FlexCols>
  </Page.BlankPage>
);

export default Home;
