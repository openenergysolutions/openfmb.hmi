import { Action, createReducer, on } from '@ngrx/store';
import * as authActions from '../actions/auth.actions';


export const authFeatureKey = 'auth';

export interface State {
  isAuthenticated?: boolean;
}

export const initialState: State = {
  isAuthenticated: false
};

const authReducer = createReducer(
  initialState,
  on(
    authActions.loginSuccess,
    authActions.validateTokenSuccess,
    (state, { data }) => ({
      ...state,
      isAuthenticated: true
    })
  )
);

export function reducer(state: State | undefined, action: Action) {
  return authReducer(state, action);
}
