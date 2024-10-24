import React from 'react';
import { useComposes } from './ComposeContext';
import ComposeItem from './ComposeItem';

const ComposeList: React.FC = () => {
  const { composes } = useComposes();

  return (
    <div>
      <h2>Compose List</h2>
      {composes.length > 0 ? (
        composes.map((compose, index) => (
          <ComposeItem
            key={index}
            id={compose.id}
            name={compose.name}
            description={compose.description}
            onSelect={(id) => console.log(`Selected compose with id: ${id}`)}
          />
        ))
      ) : (
        <p>No composes available.</p>
      )}
    </div>
  );
};

export default ComposeList;