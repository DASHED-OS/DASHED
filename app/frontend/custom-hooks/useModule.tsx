import { useState, useEffect, useCallback } from 'react';

interface Module {
  id: string;
  name: string;
  data: any;
}

const useModule = (initialModules: Module[] = []) => {
  const [modules, setModules] = useState<Module[]>(initialModules);

  const addModule = useCallback((module: Module) => {
    setModules((prevModules) => [...prevModules, module]);
  }, []);

  const removeModule = useCallback((id: string) => {
    setModules((prevModules) => prevModules.filter(module => module.id !== id));
  }, []);

  const updateModule = useCallback((id: string, newData: any) => {
    setModules((prevModules) =>
      prevModules.map(module =>
        module.id === id ? { ...module, data: newData } : module
      )
    );
  }, []);

  useEffect(() => {
    // Example effect: log modules to console whenever they change
    console.log('Modules updated:', modules);
  }, [modules]);

  return {
    modules,
    addModule,
    removeModule,
    updateModule,
  };
};

export default useModule;