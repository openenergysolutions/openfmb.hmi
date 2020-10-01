import {
  ActionReducer,
  ActionReducerMap,
  createFeatureSelector,
  createSelector,
  MetaReducer
} from '@ngrx/store';
import { environment } from '../../../environments/environment';
import * as fromAuth from './auth.reducer';
import * as fromDesigner from './designer.reducer';

export interface State {

  [fromAuth.authFeatureKey]: fromAuth.State;
  [fromDesigner.designerFeatureKey]: fromDesigner.State;
}

export const reducers: ActionReducerMap<State> = {

  [fromAuth.authFeatureKey]: fromAuth.reducer,
  [fromDesigner.designerFeatureKey]: fromDesigner.reducer,
};


export const metaReducers: MetaReducer<State>[] = !environment.production ? [] : [];
