// Type for a generic API response
export interface ApiResponse<T> {
    data: T;
    error?: string;
  }
  
  // Type for user authentication state
  export interface AuthState {
    isSignedIn: boolean;
    accountId: string;
  }
  
  // Type for a NEAR wallet
  export interface NearWallet {
    startUp: () => Promise<boolean>;
    signIn: () => void;
    signOut: () => void;
  }
  
  // Type for a UI component's props
  export interface SignInPromptProps {
    greeting: string;
    onClick: () => void;
  }
  
  export interface SignOutButtonProps {
    accountId: string;
    onClick: () => void;
  }
  
  // Type for customization settings
  export interface CustomizationSettings {
    darkMode: boolean;
    fontFamily: string;
    borderRadius: number;
  }