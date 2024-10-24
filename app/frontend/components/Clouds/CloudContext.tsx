import React, { createContext, useContext, useState, ReactNode } from 'react';

// Define the shape of the cloud context
interface CloudContextType {
  clouds: string[];
  addCloud: (cloud: string) => void;
  removeCloud: (cloud: string) => void;
}

// Create the context with a default value
const CloudContext = createContext<CloudContextType | undefined>(undefined);

// Create a provider component
export const CloudProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [clouds, setClouds] = useState<string[]>([]);

  const addCloud = (cloud: string) => {
    setClouds((prevClouds) => [...prevClouds, cloud]);
  };

  const removeCloud = (cloud: string) => {
    setClouds((prevClouds) => prevClouds.filter(c => c !== cloud));
  };

  return (
    <CloudContext.Provider value={{ clouds, addCloud, removeCloud }}>
      {children}
    </CloudContext.Provider>
  );
};

// Create a custom hook to use the CloudContext
export const useClouds = () => {
  const context = useContext(CloudContext);
  if (!context) {
    throw new Error('useClouds must be used within a CloudProvider');
  }
  return context;
};