import { createTheme } from '@mui/material/styles';

const lightTheme = createTheme({
  palette: {
    mode: 'light',
    primary: {
      main: '#1976d2',
    },
    secondary: {
      main: '#dc004e',
    },
    background: {
      default: '#ffffff',
      paper: '#f4f4f9',
    },
    text: {
      primary: '#000000',
      secondary: '#4f4f4f',
    },
  },
  typography: {
    fontFamily: `'Roboto', sans-serif`,
    h1: {
      fontSize: '2rem',
      fontWeight: 600,
    },
    h2: {
      fontSize: '1.75rem',
      fontWeight: 600,
    },
    body1: {
      fontSize: '1rem',
    },
  },
  shape: {
    borderRadius: 12,
  },
});

export default lightTheme;