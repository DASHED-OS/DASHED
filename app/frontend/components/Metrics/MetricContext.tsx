import React, { createContext, useContext, useState, ReactNode } from 'react';

// Define the shape of the metric context
interface MetricContextType {
  metrics: Array<{ id: string; name: string; value: number }>;
  addMetric: (metric: { id: string; name: string; value: number }) => void;
  removeMetric: (id: string) => void;
}

// Create the context with a default value
const MetricContext = createContext<MetricContextType | undefined>(undefined);

// Create a provider component
export const MetricProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [metrics, setMetrics] = useState<Array<{ id: string; name: string; value: number }>>([]);

  const addMetric = (metric: { id: string; name: string; value: number }) => {
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

// Create a custom hook to use the MetricContext
export const useMetrics = () => {
  const context = useContext(MetricContext);
  if (!context) {
    throw new Error('useMetrics must be used within a MetricProvider');
  }
  return context;
};