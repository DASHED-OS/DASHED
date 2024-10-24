import React, { useState, ReactNode } from 'react';
import { ClusterContext } from './ClusterContext';

interface ClusterProviderProps {
  children: ReactNode;
}

interface Cluster {
  id: string;
  name: string;
  description: string;
}

const ClusterProvider: React.FC<ClusterProviderProps> = ({ children }) => {
  const [clusters, setClusters] = useState<Cluster[]>([]);

  const addCluster = (cluster: Cluster) => {
    setClusters((prevClusters) => [...prevClusters, cluster]);
  };

  const removeCluster = (id: string) => {
    setClusters((prevClusters) => prevClusters.filter(cluster => cluster.id !== id));
  };

  return (
    <ClusterContext.Provider value={{ clusters, addCluster, removeCluster }}>
      {children}
    </ClusterContext.Provider>
  );
};

export default ClusterProvider;