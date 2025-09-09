// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Injectable, inject } from "@angular/core";
import { Router, ActivatedRouteSnapshot, Params } from "@angular/router";

interface IRoutePart {
  title: string;
  breadcrumb: string;
  params?: Params;
  url: string;
  urlSegments: any[];
}

@Injectable()
export class RoutePartsService {
  private router = inject(Router);

  public routeParts: IRoutePart[];

  generateRouteParts(snapshot: ActivatedRouteSnapshot): IRoutePart[] {
    let routeParts = <IRoutePart[]>[];
    if (snapshot) {
      if (snapshot.firstChild) {
        routeParts = routeParts.concat(
          this.generateRouteParts(snapshot.firstChild),
        );
      }
      if (snapshot.data["title"] && snapshot.url.length) {
        // console.log(snapshot.data['title'], snapshot.url)
        routeParts.push({
          title: snapshot.data["title"],
          breadcrumb: snapshot.data["breadcrumb"],
          url: snapshot.url[0].path,
          urlSegments: snapshot.url,
          params: snapshot.params,
        });
      }
    }
    return routeParts;
  }
}
