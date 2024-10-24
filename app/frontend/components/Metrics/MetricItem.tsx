import React from 'react';

interface MetricItemProps {
  id: string;
  name: string;
  value: number;
  onSelect: (id: string) => void;
}

const MetricItem: React.FC<MetricItemProps> = ({ id, name, value, onSelect }) => {
  return (
    <div onClick={() => onSelect(id)} style={{ border: '1px solid #ccc', padding: '10px', margin: '10px 0', cursor: 'pointer' }}>
      <h3>{name}</h3>
      <p>Value: {value}</p>
    </div>
  );
};

export default MetricItem;