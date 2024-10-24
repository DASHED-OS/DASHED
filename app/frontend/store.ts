import { createStore, applyMiddleware, combineReducers } from 'redux';
import thunk from 'redux-thunk';
import { composeWithDevTools } from 'redux-devtools-extension';

// Example reducer
const initialState = {
  customization: {
    darkMode: false,
    fontFamily: `'Roboto', sans-serif`,
    borderRadius: 12,
  },
};

const customizationReducer = (state = initialState.customization, action: any) => {
  switch (action.type) {
    case 'TOGGLE_DARK_MODE':
      return { ...state, darkMode: !state.darkMode };
    case 'SET_FONT_FAMILY':
      return { ...state, fontFamily: action.payload };
    case 'SET_BORDER_RADIUS':
      return { ...state, borderRadius: action.payload };
    default:
      return state;
  }
};

// Combine reducers
const rootReducer = combineReducers({
  customization: customizationReducer,
  // Add other reducers here
});

// Create store with middleware
const store = createStore(
  rootReducer,
  composeWithDevTools(applyMiddleware(thunk))
);

export default store;