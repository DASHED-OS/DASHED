import React, { createContext, useContext, useState, ReactNode } from 'react';

// Define the shape of the compose context
interface ComposeContextType {
  composes: string[];
  addCompose: (compose: string) => void;
  removeCompose: (compose: string) => void;
}

// Create the context with a default value
const ComposeContext = createContext<ComposeContextType | undefined>(undefined);

// Create a provider component
export const ComposeProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [composes, setComposes] = useState<string[]>([]);

  const addCompose = (compose: string) => {
    setComposes((prevComposes) => [...prevComposes, compose]);
  };

  const removeCompose = (compose: string) => {
    setComposes((prevComposes) => prevComposes.filter(c => c !== compose));
  };

  return (
    <ComposeContext.Provider value={{ composes, addCompose, removeCompose }}>
      {children}
    </ComposeContext.Provider>
  );
};

// Create a custom hook to use the ComposeContext
export const useComposes = () => {
  const context = useContext(ComposeContext);
  if (!context) {
    throw new Error('useComposes must be used within a ComposeProvider');
  }
  return context;
};