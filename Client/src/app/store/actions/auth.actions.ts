// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { createAction, props } from '@ngrx/store';

export const validateToken = createAction(
  '[Auth] Validate Token'
);

export const validateTokenSuccess = createAction(
  '[Auth] Validate Token Success',
  props<{ data: any }>()
);

export const validateTokenFailure = createAction(
  '[Auth] Validate Token Failure',
  props<{ error: any }>()
);

export const login = createAction(
  '[Auth] Login',
  props<{ username: any, password: any }>()
);

export const loginSuccess = createAction(
  '[Auth] Login Success',
  props<{ data: any }>()
);

export const loginFailure = createAction(
  '[Auth] Login Failure',
  props<{ error: any }>()
);
