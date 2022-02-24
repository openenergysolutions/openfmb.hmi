// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Injectable } from '@angular/core';
import { Actions, createEffect } from '@ngrx/effects';

@Injectable()
export class HmiEffects {
  constructor(private actions$: Actions) {}
}
