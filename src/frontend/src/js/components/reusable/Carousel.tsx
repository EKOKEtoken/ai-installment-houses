import * as React from 'react';
import * as Icon from 'react-feather';
import useEmblaCarousel from 'embla-carousel-react';

import Container from './Container';

interface Props {
  className?: string;
  images: string[];
}

const Carousel = ({ images, className }: Props) => {
  const [emblaRef, emblaApi] = useEmblaCarousel({ loop: true });

  const setPreviousImage = React.useCallback(
    () => emblaApi && emblaApi.scrollPrev(),
    [emblaApi],
  );

  const setNextImage = React.useCallback(
    () => emblaApi && emblaApi.scrollNext(),
    [emblaApi],
  );

  const scrollTo = React.useCallback(
    (index: number) => emblaApi && emblaApi.scrollTo(index),
    [emblaApi],
  );

  return (
    <Container.FlexCols className={`${className} gap-2`}>
      <Container.FlexRow className="justify-between bg-gray-700 shadow-lg items-center px-4">
        <Container.Container>
          <Icon.ArrowLeft
            onClick={setPreviousImage}
            size={32}
            className="text-brandLight text-xl hover:text-gray-300 cursor-pointer"
          />
        </Container.Container>
        <div className="overflow-hidden" ref={emblaRef}>
          <Container.Flex>
            {images.map((image, index) => (
              <Container.Flex
                key={index}
                className="flex-[0_0_100%] justify-center"
              >
                <img
                  className="w-auto max-h-[500px] sm:max-h-[200px]"
                  loading="lazy"
                  src={image}
                  alt="Carousel"
                  height={500}
                  width={768}
                />
              </Container.Flex>
            ))}
          </Container.Flex>
        </div>
        <Container.Container>
          <Icon.ArrowRight
            onClick={setNextImage}
            size={32}
            className="text-brandLight text-xl hover:text-gray-300 cursor-pointer"
          />
        </Container.Container>
      </Container.FlexRow>
      <Container.Container className="w-full">
        <Container.FlexRow className="items-center gap-4">
          {images.map((image, index) => (
            <Container.Container
              key={index}
              className={`w-auto h-24 border shadow-lg cursor-pointer`}
              onClick={() => {
                scrollTo(index);
              }}
            >
              <img
                className="w-full h-full"
                loading="lazy"
                height={24}
                width={300}
                src={image}
                alt="Carousel"
              />
            </Container.Container>
          ))}
        </Container.FlexRow>
      </Container.Container>
    </Container.FlexCols>
  );
};

export default Carousel;
