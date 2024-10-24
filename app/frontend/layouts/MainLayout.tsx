import React from 'react';
import { CssBaseline, Container } from '@mui/material';
import DashboardHeader from '../components/Dashboard/DashboardHeader';
import DashboardSidebar from '../components/Dashboard/DashboardSidebar';
import DashboardFooter from '../components/Dashboard/DashboardFooter';

interface MainLayoutProps {
  children: React.ReactNode;
}

const MainLayout: React.FC<MainLayoutProps> = ({ children }) => {
  return (
    <div style={{ display: 'flex', minHeight: '100vh', flexDirection: 'column' }}>
      <CssBaseline />
      <DashboardHeader />
      <div style={{ display: 'flex', flex: 1 }}>
        <DashboardSidebar />
        <Container style={{ flex: 1, padding: '20px' }}>
          {children}
        </Container>
      </div>
      <DashboardFooter />
    </div>
  );
};

export default MainLayout;