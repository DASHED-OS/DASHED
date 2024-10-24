import React from 'react';

interface CloudItemProps {
  id: string;
  name: string;
  description: string;
  onSelect: (id: string) => void;
}

const CloudItem: React.FC<CloudItemProps> = ({ id, name, description, onSelect }) => {
  return (
    <div onClick={() => onSelect(id)} style={{ border: '1px solid #ccc', padding: '10px', margin: '10px 0', cursor: 'pointer' }}>
      <h3>{name}</h3>
      <p>{description}</p>
    </div>
  );
};

export default CloudItem;