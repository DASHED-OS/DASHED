import React from 'react';

interface DashboardHeaderProps {
  title: string;
  user: {
    name: string;
    avatarUrl: string;
  };
  onSignOut: () => void;
}

const DashboardHeader: React.FC<DashboardHeaderProps> = ({ title, user, onSignOut }) => {
  return (
    <header style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', padding: '10px', borderBottom: '1px solid #ccc' }}>
      <h1>{title}</h1>
      <div style={{ display: 'flex', alignItems: 'center' }}>
        <img src={user.avatarUrl} alt={`${user.name}'s avatar`} style={{ width: '40px', height: '40px', borderRadius: '50%', marginRight: '10px' }} />
        <span>{user.name}</span>
        <button onClick={onSignOut} style={{ marginLeft: '20px' }}>Sign Out</button>
      </div>
    </header>
  );
};

export default DashboardHeader;