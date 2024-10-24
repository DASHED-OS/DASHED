import React from 'react';

interface ClusterMetricsProps {
  clusterId: string;
  metrics: {
    cpuUsage: number;
    memoryUsage: number;
    networkTraffic: number;
  };
}

const ClusterMetrics: React.FC<ClusterMetricsProps> = ({ clusterId, metrics }) => {
  return (
    <div>
      <h2>Metrics for Cluster {clusterId}</h2>
      <ul>
        <li><strong>CPU Usage:</strong> {metrics.cpuUsage}%</li>
        <li><strong>Memory Usage:</strong> {metrics.memoryUsage} MB</li>
        <li><strong>Network Traffic:</strong> {metrics.networkTraffic} Mbps</li>
      </ul>
    </div>
  );
};

export default ClusterMetrics;