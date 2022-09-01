import { useState, useEffect } from 'react';

export enum ColorMode {
  Dark,
  Light,
}

const useColourMode = () => {
  const [colourMode, setColourMode] = useState<ColorMode>(ColorMode.Dark);

  useEffect(() => {
    if (!window.matchMedia) {
      setColourMode(ColorMode.Light);
      return;
    }

    const updateColor = () => {
      if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
        setColourMode(ColorMode.Dark);
      } else {
        setColourMode(ColorMode.Light);
      }
    };

    window
      .matchMedia('(prefers-color-scheme: dark)')
      .addEventListener('change', updateColor);

    return () => {
      window
        .matchMedia('(prefers-color-scheme: dark)')
        .removeEventListener('change', updateColor);
    };
  }, []);

  return colourMode;
};

export default useColourMode;
