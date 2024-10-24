import React from 'react';

interface CloudDetailsProps {
  name: string;
  description: string;
  status: string;
}

const CloudDetails: React.FC<CloudDetailsProps> = ({ name, description, status }) => {
  return (
    <div>
      <h2>Cloud Details</h2>
      <p><strong>Name:</strong> {name}</p>
      <p><strong>Description:</strong> {description}</p>
      <p><strong>Status:</strong> {status}</p>
    </div>
  );
};

export default CloudDetails;