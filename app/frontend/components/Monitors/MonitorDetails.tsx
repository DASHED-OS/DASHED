import React from 'react';

interface Monitor {
  id: string;
  name: string;
  status: string;
  description?: string;
}

interface MonitorDetailsProps {
  monitor: Monitor;
}

const MonitorDetails: React.FC<MonitorDetailsProps> = ({ monitor }) => {
  return (
    <div>
      <h2>Monitor Details</h2>
      <p><strong>ID:</strong> {monitor.id}</p>
      <p><strong>Name:</strong> {monitor.name}</p>
      <p><strong>Status:</strong> {monitor.status}</p>
      {monitor.description && <p><strong>Description:</strong> {monitor.description}</p>}
    </div>
  );
};

export default MonitorDetails;