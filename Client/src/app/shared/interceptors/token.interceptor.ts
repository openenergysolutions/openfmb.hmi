// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Injectable, inject } from "@angular/core";
import {
  HttpEvent,
  HttpInterceptor,
  HttpHandler,
  HttpRequest,
} from "@angular/common/http";
import { Observable } from "rxjs";
import { JwtAuthService } from "../services/auth/jwt-auth.service";

@Injectable()
export class TokenInterceptor implements HttpInterceptor {
  private jwtAuth = inject(JwtAuthService);


  intercept(
    req: HttpRequest<any>,
    next: HttpHandler,
  ): Observable<HttpEvent<any>> {
    const token = this.jwtAuth.token || this.jwtAuth.getJwtToken();

    let changedReq;

    if (token) {
      changedReq = req.clone({
        setHeaders: {
          Authorization: `Bearer ${token}`,
        },
      });
    } else {
      changedReq = req;
    }
    return next.handle(changedReq);
  }
}
