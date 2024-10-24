import React from 'react';

interface ClusterDetailsProps {
  id: string;
  name: string;
  description: string;
  status: string;
}

const ClusterDetails: React.FC<ClusterDetailsProps> = ({ id, name, description, status }) => {
  return (
    <div>
      <h2>Cluster Details</h2>
      <p><strong>ID:</strong> {id}</p>
      <p><strong>Name:</strong> {name}</p>
      <p><strong>Description:</strong> {description}</p>
      <p><strong>Status:</strong> {status}</p>
    </div>
  );
};

export default ClusterDetails;