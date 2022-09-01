import { useState, useEffect } from 'react';

export enum ColourMode {
  Dark,
  Light,
}

const useColourMode = () => {
  const [colourMode, setColourMode] = useState<ColourMode>(ColourMode.Dark);

  useEffect(() => {
    if (!window.matchMedia) {
      setColourMode(ColourMode.Light);
      return;
    }

    const updateColor = () => {
      if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
        setColourMode(ColourMode.Dark);
      } else {
        setColourMode(ColourMode.Light);
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
