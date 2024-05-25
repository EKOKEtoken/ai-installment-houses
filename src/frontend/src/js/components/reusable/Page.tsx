import * as React from 'react';

const BlankPage = (props: React.HTMLProps<HTMLDivElement>) => {
  return (
    <div className={`${props.className} w-screen min-h-screen bg-page`}>
      <PageContent>{props.children}</PageContent>
    </div>
  );
};

const BrandPage = (props: React.HTMLProps<HTMLDivElement>) => {
  return (
    <BlankPage className={`${props.className} relative bg-brand py-32`}>
      <PageContent className="z-1 relative">{props.children}</PageContent>
    </BlankPage>
  );
};

const PageContent = (props: React.HTMLProps<HTMLDivElement>) => {
  return (
    <div
      id={props.id}
      itemProp={props.itemProp}
      itemScope={props.itemScope}
      itemType={props.itemType}
      className={`${props.className} w-page mx-auto py-[120px]`}
    >
      {props.children}
    </div>
  );
};

export default {
  BlankPage,
  BrandPage,
};
