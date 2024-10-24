import React, { useState, ReactNode } from 'react';
import { MetricContext } from './MetricContext';

interface MetricProviderProps {
  children: ReactNode;
}

interface Metric {
  id: string;
  name: string;
  value: number;
}

const MetricProvider: React.FC<MetricProviderProps> = ({ children }) => {
  const [metrics, setMetrics] = useState<Metric[]>([]);

  const addMetric = (metric: Metric) => {
    setMetrics((prevMetrics) => [...prevMetrics, metric]);
  };

  const removeMetric = (id: string) => {
    setMetrics((prevMetrics) => prevMetrics.filter(metric => metric.id !== id));
  };

  return (
    <MetricContext.Provider value={{ metrics, addMetric, removeMetric }}>
      {children}
    </MetricContext.Provider>
  );
};

export default MetricProvider;