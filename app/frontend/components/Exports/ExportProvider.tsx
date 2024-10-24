import React, { useState, ReactNode } from 'react';
import { ExportContext } from './ExportContext';

interface ExportProviderProps {
  children: ReactNode;
}

interface ExportItem {
  id: string;
  name: string;
  description: string;
}

const ExportProvider: React.FC<ExportProviderProps> = ({ children }) => {
  const [exports, setExports] = useState<ExportItem[]>([]);

  const addExport = (exportItem: ExportItem) => {
    setExports((prevExports) => [...prevExports, exportItem]);
  };

  const removeExport = (id: string) => {
    setExports((prevExports) => prevExports.filter(exportItem => exportItem.id !== id));
  };

  return (
    <ExportContext.Provider value={{ exports, addExport, removeExport }}>
      {children}
    </ExportContext.Provider>
  );
};

export default ExportProvider;