// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Injectable, inject } from "@angular/core";
import {
  HttpRequest,
  HttpHandler,
  HttpEvent,
  HttpInterceptor,
} from "@angular/common/http";
import { Observable } from "rxjs";
import { finalize } from "rxjs/operators";
import { NgxSpinnerService } from "ngx-spinner";

@Injectable()
export class LoadingInterceptor implements HttpInterceptor {
  private spinner = inject(NgxSpinnerService);


  intercept(
    request: HttpRequest<any>,
    next: HttpHandler,
  ): Observable<HttpEvent<any>> {
    this.spinner.show();
    return next.handle(request).pipe(finalize(() => this.spinner.hide()));
  }
}
