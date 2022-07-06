// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

// TODO: Cleanup comments
import {
  ActionReducerMap,
  // createFeatureSelector,
  // createSelector,
  MetaReducer
} from '@ngrx/store';
import { environment } from '../../../environments/environment';
import * as fromDesigner from './designer.reducer';
import * as fromHmi from './hmi.reducer';

export interface State {

  // [fromAuth.authFeatureKey]: fromAuth.State;
  [fromDesigner.designerFeatureKey]: fromDesigner.State;
  [fromHmi.hmiFeatureKey]: fromHmi.CommState;
}

export const reducers: ActionReducerMap<State> = {

  // [fromAuth.authFeatureKey]: fromAuth.reducer,
  [fromDesigner.designerFeatureKey]: fromDesigner.reducer,
  [fromHmi.hmiFeatureKey]: fromHmi.reducer
};


export const metaReducers: MetaReducer<State>[] = !environment.production ? [] : [];
