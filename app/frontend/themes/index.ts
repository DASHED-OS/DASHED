import lightTheme from './lightTheme';
import darkTheme from './darkTheme';

const themes = (customization: any) => {
  return customization.darkMode ? darkTheme : lightTheme;
};

export default themes;