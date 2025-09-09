// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Component, Input } from "@angular/core";

@Component({
    selector: "app-sidenav",
    templateUrl: "./sidenav.template.html"
})
export class SidenavComponent {
  @Input() public menuItems: any[] = [];
  @Input() public hasIconTypeMenuItem: boolean;
  @Input() public iconTypeMenuTitle: string;

  constructor() {}
}
