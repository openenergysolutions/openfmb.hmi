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
import { MatSnackBar } from "@angular/material/snack-bar";

@Injectable()
export class UserRoleGuard {
  private router = inject(Router);
  private jwtAuth = inject(JwtAuthService);
  private snack = inject(MatSnackBar);


  canActivate(route: ActivatedRouteSnapshot, _: RouterStateSnapshot) {
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
