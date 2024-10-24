import React from 'react';

interface ImportDetailsProps {
  id: string;
  name: string;
  description: string;
  status: string;
}

const ImportDetails: React.FC<ImportDetailsProps> = ({ id, name, description, status }) => {
  return (
    <div>
      <h2>Import Details</h2>
      <p><strong>ID:</strong> {id}</p>
      <p><strong>Name:</strong> {name}</p>
      <p><strong>Description:</strong> {description}</p>
      <p><strong>Status:</strong> {status}</p>
    </div>
  );
};

export default ImportDetails;