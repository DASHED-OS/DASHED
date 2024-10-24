import React, { createContext, useContext, useState, ReactNode } from 'react';

// Define the shape of the dashboard context
interface DashboardContextType {
  widgets: string[];
  addWidget: (widget: string) => void;
  removeWidget: (widget: string) => void;
}

// Create the context with a default value
const DashboardContext = createContext<DashboardContextType | undefined>(undefined);

// Create a provider component
export const DashboardProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [widgets, setWidgets] = useState<string[]>([]);

  const addWidget = (widget: string) => {
    setWidgets((prevWidgets) => [...prevWidgets, widget]);
  };

  const removeWidget = (widget: string) => {
    setWidgets((prevWidgets) => prevWidgets.filter(w => w !== widget));
  };

  return (
    <DashboardContext.Provider value={{ widgets, addWidget, removeWidget }}>
      {children}
    </DashboardContext.Provider>
  );
};

// Create a custom hook to use the DashboardContext
export const useDashboard = () => {
  const context = useContext(DashboardContext);
  if (!context) {
    throw new Error('useDashboard must be used within a DashboardProvider');
  }
  return context;
};