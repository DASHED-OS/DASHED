// Function to format a date to a readable string
export const formatDate = (date: Date): string => {
    return date.toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    });
  };
  
  // Function to capitalize the first letter of a string
  export const capitalizeFirstLetter = (str: string): string => {
    if (!str) return '';
    return str.charAt(0).toUpperCase() + str.slice(1);
  };
  
  // Function to check if an object is empty
  export const isEmptyObject = (obj: Record<string, any>): boolean => {
    return Object.keys(obj).length === 0;
  };
  
  // Function to generate a random ID
  export const generateRandomId = (): string => {
    return Math.random().toString(36).substr(2, 9);
  };
  
  // Function to debounce another function
  export const debounce = (func: Function, delay: number) => {
    let timeoutId: NodeJS.Timeout;
    return (...args: any[]) => {
      if (timeoutId) clearTimeout(timeoutId);
      timeoutId = setTimeout(() => {
        func(...args);
      }, delay);
    };
  };