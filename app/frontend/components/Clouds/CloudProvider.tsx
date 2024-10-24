import React, { useState, ReactNode } from 'react';
import { CloudContext } from './CloudContext';

interface CloudProviderProps {
  children: ReactNode;
}

const CloudProvider: React.FC<CloudProviderProps> = ({ children }) => {
  const [clouds, setClouds] = useState<{ id: string; name: string; description: string }[]>([]);

  const addCloud = (cloud: { id: string; name: string; description: string }) => {
    setClouds((prevClouds) => [...prevClouds, cloud]);
  };

  const removeCloud = (id: string) => {
    setClouds((prevClouds) => prevClouds.filter(cloud => cloud.id !== id));
  };

  return (
    <CloudContext.Provider value={{ clouds, addCloud, removeCloud }}>
      {children}
    </CloudContext.Provider>
  );
};

export default CloudProvider;