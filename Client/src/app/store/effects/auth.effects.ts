// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Injectable, inject } from "@angular/core";
import { Actions, createEffect, ofType } from "@ngrx/effects";
import * as authActions from "../actions/auth.actions";
import { switchMap, map, tap } from "rxjs/operators";
import { HttpClient } from "@angular/common/http";
import { AuthenticationService } from "src/app/core/services/authentication.service";
import { Router } from "@angular/router";

@Injectable()
export class AuthEffects {
  private http = inject(HttpClient);
  private actions$ = inject(Actions);
  private authenticationService = inject(AuthenticationService);
  private router = inject(Router);


  validateToken$ = createEffect(() =>
    this.actions$.pipe(
      ofType(authActions.validateToken),
      switchMap(() => {
        return this.authenticationService.validate().pipe(
          map((data) => {
            return authActions.validateTokenSuccess({ data });
          }),
        );
      }),
    ),
  );

  login$ = createEffect(() =>
    this.actions$.pipe(
      ofType(authActions.login),
      switchMap(({ username, password }) => {
        return this.authenticationService.login(username, password).pipe(
          map((data) => {
            return authActions.loginSuccess({ data });
          }),
        );
      }),
    ),
  );

  validateTokenSuccess$ = createEffect(
    () =>
      this.actions$.pipe(
        ofType(authActions.validateTokenSuccess, authActions.loginSuccess),
        tap(() => this.router.navigate(["/"])),
      ),
    { dispatch: false },
  );
}
