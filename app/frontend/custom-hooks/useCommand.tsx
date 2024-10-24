import { useState, useCallback } from 'react';

interface Command {
  id: string;
  execute: () => void;
}

const useCommands = () => {
  const [commands, setCommands] = useState<Command[]>([]);

  const addCommand = useCallback((command: Command) => {
    setCommands((prevCommands) => [...prevCommands, command]);
  }, []);

  const removeCommand = useCallback((id: string) => {
    setCommands((prevCommands) => prevCommands.filter(command => command.id !== id));
  }, []);

  const executeCommand = useCallback((id: string) => {
    const command = commands.find(command => command.id === id);
    if (command) {
      command.execute();
    }
  }, [commands]);

  return {
    commands,
    addCommand,
    removeCommand,
    executeCommand,
  };
};

export default useCommands;