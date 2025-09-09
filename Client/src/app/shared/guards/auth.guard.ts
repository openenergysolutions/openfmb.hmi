// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Injectable, inject } from "@angular/core";
import {
  ActivatedRouteSnapshot,
  RouterStateSnapshot,
  Router,
} from "@angular/router";
import { JwtAuthService } from "../services/auth/jwt-auth.service";

@Injectable()
export class AuthGuard {
  private router = inject(Router);
  private jwtAuth = inject(JwtAuthService);


  canActivate(route: ActivatedRouteSnapshot, state: RouterStateSnapshot) {
    if (this.jwtAuth.isLoggedIn()) {
      return true;
    } else {
      this.router.navigate(["/sessions/signin"], {
        queryParams: {
          return: state.url,
        },
      });
      return false;
    }
  }
}
