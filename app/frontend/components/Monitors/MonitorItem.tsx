import React from 'react';

interface MonitorItemProps {
  id: string;
  name: string;
  status: string;
  onSelect: (id: string) => void;
}

const MonitorItem: React.FC<MonitorItemProps> = ({ id, name, status, onSelect }) => {
  return (
    <div onClick={() => onSelect(id)} style={{ border: '1px solid #ccc', padding: '10px', margin: '10px 0', cursor: 'pointer' }}>
      <h3>{name}</h3>
      <p>Status: {status}</p>
    </div>
  );
};

export default MonitorItem;