import React from 'react';
import DashboardHeader from './DashboardHeader';
import DashboardContent from './DashboardContent';
import DashboardSidebar from './DashboardSidebar';
import DashboardWidgets from './DashboardWidgets';

const DashboardHome: React.FC = () => {
  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100vh' }}>
      <DashboardHeader title="Dashboard" user={{ name: 'John Doe', avatarUrl: 'https://example.com/avatar.jpg' }} onSignOut={() => console.log('Sign out')} />
      <div style={{ display: 'flex', flex: 1 }}>
        <DashboardSidebar />
        <DashboardContent>
          <DashboardWidgets />
        </DashboardContent>
      </div>
    </div>
  );
};

export default DashboardHome;