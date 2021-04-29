// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { validateTokenSuccess, loginSuccess } from './../actions/auth.actions';
import { Injectable } from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';
import * as authActions from '../actions/auth.actions';
import { switchMap, map, tap } from 'rxjs/operators';
import { HttpClient } from '@angular/common/http';
import { AuthenticationService } from 'src/app/core/services/authentication.service';
import { Router } from '@angular/router';


@Injectable()
export class AuthEffects {

  constructor(
    private http: HttpClient,
    private actions$: Actions,
    private authenticationService: AuthenticationService,
    private router: Router
  ) { }

  validateToken$ = createEffect(() =>
    this.actions$.pipe(
      ofType(authActions.validateToken),
      switchMap(() => {
        return this.authenticationService.validate().pipe(
          map(data => {
            return authActions.validateTokenSuccess({ data });
          })
        );
      })
    )
  );

  login$ = createEffect(() =>
    this.actions$.pipe(
      ofType(authActions.login),
      switchMap(({ username, password }) => {
        return this.authenticationService.login(username, password).pipe(
          map(data => {
            return authActions.loginSuccess({ data });
          })
        );
      })
    )
  );

  validateTokenSuccess$ = createEffect(() =>
    this.actions$.pipe(
      ofType(
        authActions.validateTokenSuccess,
        authActions.loginSuccess
      ),
      tap(() => this.router.navigate(['/']))
    ), { dispatch: false }
  );
}
