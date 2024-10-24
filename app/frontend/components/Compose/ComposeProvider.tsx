import React, { useState, ReactNode } from 'react';
import { ComposeContext } from './ComposeContext';

interface ComposeProviderProps {
  children: ReactNode;
}

interface Compose {
  id: string;
  name: string;
  description: string;
}

const ComposeProvider: React.FC<ComposeProviderProps> = ({ children }) => {
  const [composes, setComposes] = useState<Compose[]>([]);

  const addCompose = (compose: Compose) => {
    setComposes((prevComposes) => [...prevComposes, compose]);
  };

  const removeCompose = (id: string) => {
    setComposes((prevComposes) => prevComposes.filter(compose => compose.id !== id));
  };

  return (
    <ComposeContext.Provider value={{ composes, addCompose, removeCompose }}>
      {children}
    </ComposeContext.Provider>
  );
};

export default ComposeProvider;