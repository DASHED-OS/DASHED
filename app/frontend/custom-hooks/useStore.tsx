import { useState, useEffect, useCallback } from 'react';

const useStart = (initialState: boolean = false) => {
  const [isStarted, setIsStarted] = useState(initialState);

  const start = useCallback(() => {
    setIsStarted(true);
  }, []);

  const stop = useCallback(() => {
    setIsStarted(false);
  }, []);

  useEffect(() => {
    if (isStarted) {
      console.log('Process started');
      // Add any side effects or initialization logic here
    } else {
      console.log('Process stopped');
      // Add any cleanup logic here
    }
  }, [isStarted]);

  return {
    isStarted,
    start,
    stop,
  };
};

export default useStart;