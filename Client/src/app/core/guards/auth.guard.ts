// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Injectable } from '@angular/core';
import { CanActivate, ActivatedRouteSnapshot, RouterStateSnapshot, UrlTree, Router } from '@angular/router';
import { Observable } from 'rxjs';
import { Store } from '@ngrx/store';
import { routerRequestAction, routerNavigationAction } from '@ngrx/router-store';

import * as fromRoot from '../../store/reducers/index';

@Injectable({
  providedIn: 'root'
})
export class AuthGuard implements CanActivate {

  constructor(
    private store: Store<fromRoot.State>,
    private router: Router
  ) { }

  canActivate(
    next: ActivatedRouteSnapshot,
    state: RouterStateSnapshot): Observable<boolean | UrlTree> | Promise<boolean | UrlTree> | boolean | UrlTree {

    const isAuthenticated$ = this.store.select(storeState => storeState.auth.isAuthenticated);

    isAuthenticated$.subscribe(authenticated => {
      console.log('AuthGuard::Authenticated: ' + authenticated);
      if (!authenticated) {
        this.router.navigateByUrl('login');
      }
    });

    return isAuthenticated$;
  }
}
