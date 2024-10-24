import React from 'react';

interface ComposeDetailsProps {
  id: string;
  name: string;
  description: string;
  status: string;
}

const ComposeDetails: React.FC<ComposeDetailsProps> = ({ id, name, description, status }) => {
  return (
    <div>
      <h2>Compose Details</h2>
      <p><strong>ID:</strong> {id}</p>
      <p><strong>Name:</strong> {name}</p>
      <p><strong>Description:</strong> {description}</p>
      <p><strong>Status:</strong> {status}</p>
    </div>
  );
};

export default ComposeDetails;