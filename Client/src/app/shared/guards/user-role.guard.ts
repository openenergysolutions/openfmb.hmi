// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Injectable } from "@angular/core";
import {
  ActivatedRouteSnapshot,
  RouterStateSnapshot,
  Router,
} from "@angular/router";
import { JwtAuthService } from "../services/auth/jwt-auth.service";
import { MatSnackBar } from "@angular/material/snack-bar";

@Injectable()
export class UserRoleGuard {
  constructor(
    private router: Router,
    private jwtAuth: JwtAuthService,
    private snack: MatSnackBar,
  ) {}

  canActivate(route: ActivatedRouteSnapshot, _state: RouterStateSnapshot) {
    const user = this.jwtAuth.getUser();

    if (
      user &&
      route.data &&
      route.data.roles &&
      route.data.roles.includes(user.role)
    ) {
      return true;
    } else {
      this.snack.open("You do not have access to this page!", "OK", {
        duration: 5000,
      });
      return false;
    }
  }
}
