import React from 'react';
import { useAuth } from './AuthContext';

const Login: React.FC = () => {
  const { login } = useAuth();

  const handleLogin = () => {
    // Call the login function from the AuthContext
    login();
  };

  return (
    <div>
      <h2>Login</h2>
      <button onClick={handleLogin}>Log In</button>
    </div>
  );
};

export default Login;