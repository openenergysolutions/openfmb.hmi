// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Injectable } from "@angular/core";
import { MatSidenav } from "@angular/material/sidenav";

@Injectable({
  providedIn: "root"
})
export class SidenavHelperService {
  sidenavList: MatSidenav[];

  constructor() {
    this.sidenavList = [];
  }

  setSidenav(id, sidenav): void {
    this.sidenavList[id] = sidenav;
  }

  getSidenav(id): any {
    return this.sidenavList[id];
  }
}
