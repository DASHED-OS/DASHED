import React, { createContext, useContext, useState, ReactNode } from 'react';

// Define the shape of the export context
interface ExportContextType {
  exports: string[];
  addExport: (exportItem: string) => void;
  removeExport: (exportItem: string) => void;
}

// Create the context with a default value
const ExportContext = createContext<ExportContextType | undefined>(undefined);

// Create a provider component
export const ExportProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [exports, setExports] = useState<string[]>([]);

  const addExport = (exportItem: string) => {
    setExports((prevExports) => [...prevExports, exportItem]);
  };

  const removeExport = (exportItem: string) => {
    setExports((prevExports) => prevExports.filter(e => e !== exportItem));
  };

  return (
    <ExportContext.Provider value={{ exports, addExport, removeExport }}>
      {children}
    </ExportContext.Provider>
  );
};

// Create a custom hook to use the ExportContext
export const useExports = () => {
  const context = useContext(ExportContext);
  if (!context) {
    throw new Error('useExports must be used within an ExportProvider');
  }
  return context;
};