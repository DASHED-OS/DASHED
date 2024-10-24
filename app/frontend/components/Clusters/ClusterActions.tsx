import React from 'react';

interface ClusterActionsProps {
  onAddCluster: () => void;
  onRemoveCluster: (id: string) => void;
  onUpdateCluster: (id: string) => void;
}

const ClusterActions: React.FC<ClusterActionsProps> = ({ onAddCluster, onRemoveCluster, onUpdateCluster }) => {
  return (
    <div>
      <h2>Cluster Actions</h2>
      <button onClick={onAddCluster}>Add Cluster</button>
      <button onClick={() => onRemoveCluster('example-id')}>Remove Cluster</button>
      <button onClick={() => onUpdateCluster('example-id')}>Update Cluster</button>
    </div>
  );
};

export default ClusterActions;