import React from 'react';
import { useMetrics } from './MetricContext';
import MetricCharts from './MetricCharts';
import MetricActions from './MetricActions';

const MetricDashboard: React.FC = () => {
  const { metrics, addMetric, removeMetric } = useMetrics();

  return (
    <div>
      <h1>Metric Dashboard</h1>
      <MetricActions
        onAddMetric={() => addMetric({ id: 'new-id', name: 'New Metric', value: 0 })}
        onRemoveMetric={(id) => removeMetric(id)}
        onUpdateMetric={(id) => console.log(`Update metric with id: ${id}`)}
      />
      <MetricCharts data={metrics.map(metric => ({ name: metric.name, value: metric.value }))} />
    </div>
  );
};

export default MetricDashboard;