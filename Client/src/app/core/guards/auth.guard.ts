// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Injectable, inject } from "@angular/core";
import {
  CanActivate,
  ActivatedRouteSnapshot,
  RouterStateSnapshot,
  UrlTree,
  Router,
} from "@angular/router";
import { Observable } from "rxjs";
import { tap } from "rxjs/operators";
import { Store } from "@ngrx/store";

import * as fromRoot from "../../store/reducers/index";

@Injectable({
  providedIn: "root",
})
export class AuthGuard implements CanActivate {
  private store = inject<Store<fromRoot.State>>(Store);
  private router = inject(Router);


  canActivate(
    _: ActivatedRouteSnapshot,
    __: RouterStateSnapshot,
  ):
    | Observable<boolean | UrlTree>
    | Promise<boolean | UrlTree>
    | boolean
    | UrlTree {
    return this.store
      .select((storeState) => storeState.auth.isAuthenticated)
      .pipe(
        tap((authenticated) => {
          if (!authenticated) {
            this.router.navigateByUrl("login");
          }
        }),
      );
  }
}
