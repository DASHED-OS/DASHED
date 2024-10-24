import React from 'react';

interface DashboardSidebarProps {
  items: { name: string; onClick: () => void }[];
}

const DashboardSidebar: React.FC<DashboardSidebarProps> = ({ items }) => {
  return (
    <aside style={{ width: '250px', borderRight: '1px solid #ccc', padding: '10px' }}>
      <h2>Sidebar</h2>
      <ul style={{ listStyleType: 'none', padding: 0 }}>
        {items.map((item, index) => (
          <li key={index} style={{ marginBottom: '10px' }}>
            <button onClick={item.onClick} style={{ width: '100%', textAlign: 'left' }}>
              {item.name}
            </button>
          </li>
        ))}
      </ul>
    </aside>
  );
};

export default DashboardSidebar;