import React, { useState, ReactNode } from 'react';
import { ImportContext } from './ImportContext';

interface ImportProviderProps {
  children: ReactNode;
}

interface ImportItem {
  id: string;
  name: string;
  description: string;
}

const ImportProvider: React.FC<ImportProviderProps> = ({ children }) => {
  const [imports, setImports] = useState<ImportItem[]>([]);

  const addImport = (importItem: ImportItem) => {
    setImports((prevImports) => [...prevImports, importItem]);
  };

  const removeImport = (id: string) => {
    setImports((prevImports) => prevImports.filter(importItem => importItem.id !== id));
  };

  return (
    <ImportContext.Provider value={{ imports, addImport, removeImport }}>
      {children}
    </ImportContext.Provider>
  );
};

export default ImportProvider;