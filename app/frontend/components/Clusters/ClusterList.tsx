import React from 'react';
import { useClusters } from './ClusterContext';
import ClusterItem from './ClusterItem';

const ClusterList: React.FC = () => {
  const { clusters } = useClusters();

  return (
    <div>
      <h2>Cluster List</h2>
      {clusters.length > 0 ? (
        clusters.map((cluster, index) => (
          <ClusterItem
            key={index}
            id={cluster.id}
            name={cluster.name}
            description={cluster.description}
            onSelect={(id) => console.log(`Selected cluster with id: ${id}`)}
          />
        ))
      ) : (
        <p>No clusters available.</p>
      )}
    </div>
  );
};

export default ClusterList;