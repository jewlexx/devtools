import { createContext } from 'react';
import { ColourMode } from '../hooks/useColourMode';

const ColorModeContext = createContext(ColourMode.Dark);

export const ColorModeConsumer = ColorModeContext.Consumer;
export const ColorModeProvider = ColorModeContext.Provider;
