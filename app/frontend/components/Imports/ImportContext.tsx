import React, { createContext, useContext, useState, ReactNode } from 'react';

// Define the shape of the import context
interface ImportContextType {
  imports: string[];
  addImport: (importItem: string) => void;
  removeImport: (importItem: string) => void;
}

// Create the context with a default value
const ImportContext = createContext<ImportContextType | undefined>(undefined);

// Create a provider component
export const ImportProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [imports, setImports] = useState<string[]>([]);

  const addImport = (importItem: string) => {
    setImports((prevImports) => [...prevImports, importItem]);
  };

  const removeImport = (importItem: string) => {
    setImports((prevImports) => prevImports.filter(i => i !== importItem));
  };

  return (
    <ImportContext.Provider value={{ imports, addImport, removeImport }}>
      {children}
    </ImportContext.Provider>
  );
};

// Create a custom hook to use the ImportContext
export const useImports = () => {
  const context = useContext(ImportContext);
  if (!context) {
    throw new Error('useImports must be used within an ImportProvider');
  }
  return context;
};