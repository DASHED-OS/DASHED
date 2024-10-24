import React from 'react';

interface MetricActionsProps {
  onAddMetric: () => void;
  onRemoveMetric: (id: string) => void;
  onUpdateMetric: (id: string) => void;
}

const MetricActions: React.FC<MetricActionsProps> = ({ onAddMetric, onRemoveMetric, onUpdateMetric }) => {
  return (
    <div>
      <h2>Metric Actions</h2>
      <button onClick={onAddMetric}>Add Metric</button>
      <button onClick={() => onRemoveMetric('example-id')}>Remove Metric</button>
      <button onClick={() => onUpdateMetric('example-id')}>Update Metric</button>
    </div>
  );
};

export default MetricActions;