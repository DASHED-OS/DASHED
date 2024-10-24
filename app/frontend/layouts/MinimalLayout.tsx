import React from 'react';
import { CssBaseline, Container } from '@mui/material';

interface MinimalLayoutProps {
  children: React.ReactNode;
}

const MinimalLayout: React.FC<MinimalLayoutProps> = ({ children }) => {
  return (
    <div style={{ display: 'flex', flexDirection: 'column', minHeight: '100vh' }}>
      <CssBaseline />
      <Container style={{ flex: 1, padding: '20px' }}>
        {children}
      </Container>
    </div>
  );
};

export default MinimalLayout;