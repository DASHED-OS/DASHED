import React from 'react';

interface ExportDetailsProps {
  id: string;
  name: string;
  description: string;
  status: string;
}

const ExportDetails: React.FC<ExportDetailsProps> = ({ id, name, description, status }) => {
  return (
    <div>
      <h2>Export Details</h2>
      <p><strong>ID:</strong> {id}</p>
      <p><strong>Name:</strong> {name}</p>
      <p><strong>Description:</strong> {description}</p>
      <p><strong>Status:</strong> {status}</p>
    </div>
  );
};

export default ExportDetails;