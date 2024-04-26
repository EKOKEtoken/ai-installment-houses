import * as React from 'react';
import * as Icon from 'react-feather';

import Container from './reusable/Container';
import Heading from './reusable/Heading';
import Link from './reusable/Link';
import X from './svg/X';
import TikTok from './svg/TikTok';
import Telegram from './svg/Telegram';

const Footer = () => {
  const year = new Date().getFullYear();
  return (
    <footer>
      <Container.Container className="bg-brandRed text-white m-0 p-8">
        <div className="bg-white content-none h-[1px] mx-auto w-page"></div>
        <Container.FlexResponsiveRow className="justify-between w-page sm:w-full mx-auto">
          <Container.FlexCols>
            <Heading.H2 className="text-white">EKOKE Token</Heading.H2>
            <Link.Default
              href={'https://www.ekoketoken.com/'}
              className="text-white"
            >
              Home
            </Link.Default>
          </Container.FlexCols>
          <Container.FlexCols className="gap-4">
            <Heading.H2 className="text-white">Contacts</Heading.H2>
            <Link.Default
              href={'mailto:ekokefly@gmail.com'}
              className="text-white"
            >
              ekokefly@gmail.com
            </Link.Default>
            <Link.Default href={'tel:+393335294334'} className="text-white">
              +39 333 529 4334
            </Link.Default>
            <Link.Default className="text-white">
              Piazzale Brescia, 16, 20149 Milano MI, Italy
            </Link.Default>
          </Container.FlexCols>
          <Container.FlexCols className="gap-4">
            <Heading.H2 className="text-white">Follow Us On</Heading.H2>
            <Link.IconLink
              href={'https://github.com/EKOKEtoken'}
              target="_blank"
              className="text-white"
            >
              <Icon.GitHub className="mr-2 inline text-white" />
              GitHub
            </Link.IconLink>
            <Link.IconLink
              href={'https://www.instagram.com/ekoketoken'}
              target="_blank"
              className="text-white"
            >
              <Icon.Instagram className="mr-2 inline text-white" />
              Instagram
            </Link.IconLink>
            <Link.IconLink
              href={'https://www.linkedin.com/company/ekoketoken'}
              target="_blank"
              className="text-white"
            >
              <Icon.Linkedin className="mr-2 inline text-white" />
              LinkedIn
            </Link.IconLink>
            <Link.IconLink
              href={'https://t.me/ekokeTOKENgroup'}
              target="_blank"
              className="text-white"
            >
              <Telegram fill="#ffffff" />
              <span className="ml-2">Telegram</span>
            </Link.IconLink>
            <Link.IconLink
              href={'https://www.tiktok.com/@ekoketoken?lang=en'}
              target="_blank"
              className="text-white"
            >
              <TikTok fill="#ffffff" />
              <span className="ml-2">TikTok</span>
            </Link.IconLink>
            <Link.IconLink
              href={'https://twitter.com/ekoketoken'}
              target="_blank"
              className="text-white"
            >
              <X />
              <span className="ml-2">Twitter</span>
            </Link.IconLink>
          </Container.FlexCols>
        </Container.FlexResponsiveRow>
        <p className="text-xs text-center text-white my-4">
          Copyright © {year} by ekoke token | Powered and secured by ICP
          Blockchain
        </p>
      </Container.Container>
    </footer>
  );
};

export default Footer;
