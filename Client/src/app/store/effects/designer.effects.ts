// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Injectable, inject } from "@angular/core";
import { Actions } from "@ngrx/effects";

@Injectable()
export class DesignerEffects {
  private actions$ = inject(Actions);
}
