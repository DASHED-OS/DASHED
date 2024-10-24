import React from 'react';

interface Metric {
  id: string;
  name: string;
  value: number;
  description?: string;
}

interface MetricDetailsProps {
  metric: Metric;
}

const MetricDetails: React.FC<MetricDetailsProps> = ({ metric }) => {
  return (
    <div>
      <h2>Metric Details</h2>
      <p><strong>ID:</strong> {metric.id}</p>
      <p><strong>Name:</strong> {metric.name}</p>
      <p><strong>Value:</strong> {metric.value}</p>
      {metric.description && <p><strong>Description:</strong> {metric.description}</p>}
    </div>
  );
};

export default MetricDetails;