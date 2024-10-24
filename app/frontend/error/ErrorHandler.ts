class ErrorHandler {
    static logError(error: Error, errorInfo?: React.ErrorInfo) {
      // Log the error to an external service
      console.error("Logging error:", error, errorInfo);
      // You can replace this with a call to an error logging service
    }
  
    static displayErrorMessage(message: string) {
      // Display an error message to the user
      alert(message);
      // You can replace this with a more sophisticated UI notification
    }
  }
  
  export default ErrorHandler;