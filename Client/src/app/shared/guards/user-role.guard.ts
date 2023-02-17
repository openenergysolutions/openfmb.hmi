// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Injectable } from "@angular/core";
import {
  CanActivate,
  ActivatedRouteSnapshot,
  RouterStateSnapshot,
  Router,
} from "@angular/router";
import { MatSnackBar } from "@angular/material/snack-bar";
import { AuthService } from "@auth0/auth0-angular";
import { parseJwt } from "../helpers/utils";

@Injectable()
export class UserRoleGuard implements CanActivate {

  constructor(private router: Router, private auth: AuthService, private snack: MatSnackBar) { }

  async canActivate(route: ActivatedRouteSnapshot, state: RouterStateSnapshot) {

    return this.auth.getAccessTokenSilently({audience: "openfmb-hmi"}).toPromise().then(access_token => {
      const access_token_d = parseJwt(access_token);
      const roles = access_token_d.resource_access.gms.roles;
      if (route?.data?.roles.some((role: string) => roles.includes(role))) {
        return true;
      } else {
        console.log('Failed to find required role');
        this.snack.open('You are not authorized to access this page', 'OK', { duration: 5000 });
        //TODO: Fix race condition!
        // this.router.navigate([this.router.url]);
        setTimeout(this.router.navigate.bind(null, [this.router.url]), 250);
        return false;
      }
    });
  }

}
