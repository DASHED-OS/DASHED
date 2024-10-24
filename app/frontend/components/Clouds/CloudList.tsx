import React from 'react';
import { useClouds } from './CloudContext';
import CloudItem from './CloudItem';

const CloudList: React.FC = () => {
  const { clouds } = useClouds();

  return (
    <div>
      <h2>Cloud List</h2>
      {clouds.length > 0 ? (
        clouds.map((cloud, index) => (
          <CloudItem
            key={index}
            id={cloud.id}
            name={cloud.name}
            description={cloud.description}
            onSelect={(id) => console.log(`Selected cloud with id: ${id}`)}
          />
        ))
      ) : (
        <p>No clouds available.</p>
      )}
    </div>
  );
};

export default CloudList;