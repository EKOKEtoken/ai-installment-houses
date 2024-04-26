import * as React from 'react';

import Container from '../reusable/Container';
import IcConnect from '../IcConnect';

const Mobile = () => (
  <div className="h-[80px] items-center hidden sm:flex left-0 gap-4 justify-start py-2 px-4 top-0 w-full z-40">
    <Container.FlexRow className="items-center justify-between w-full">
      <Container.Container className="border-brandRed border-b-4 py-2">
        <span className="font-medium text-brand">EKOKE Marketplace</span>
      </Container.Container>
      <IcConnect />
    </Container.FlexRow>
  </div>
);

export default Mobile;
