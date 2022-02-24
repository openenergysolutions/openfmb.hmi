// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Action, createReducer, on } from '@ngrx/store';
import * as hmiActions from '../actions/hmi.actions';

export const hmiFeatureKey = 'hmi';

export class CommunicationStatus {
  public static UNKNOWN = 0;
  public static OK = 1;
  public static NOT_OK = 2;
}

export interface CommState {
  status: number;
}

export const initialState: CommState = {
  status: CommunicationStatus.UNKNOWN,
};

const hmiReducer = createReducer(
  initialState,
  on(
    hmiActions.commStatus,
    (state, { status }) => ({
      ...state,
      status
    })
  )
);

export function reducer(state: CommState | undefined, action: Action) {
  return hmiReducer(state, action);
}
