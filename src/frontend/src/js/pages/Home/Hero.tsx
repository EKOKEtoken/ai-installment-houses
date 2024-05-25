import * as React from 'react';

import Jumbotron from '../../components/reusable/Jumbotron';
import Heading from '../../components/reusable/Heading';

const Hero = () => (
  <Jumbotron className="bg-hero bg-top bg-fixed bg-cover w-10/12 sm:w-full">
    <Heading.H1 className="text-brand text-center">
      Welcome to the Home Page
    </Heading.H1>
  </Jumbotron>
);

export default Hero;
