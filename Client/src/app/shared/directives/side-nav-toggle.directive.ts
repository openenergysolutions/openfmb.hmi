// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import {
  Directive,
  Host,
  Self,
  Optional,
  OnDestroy,
  OnInit,
} from "@angular/core";
import { MediaChange, MediaObserver } from "ngx-flexible-layout";
import { Subscription } from "rxjs";
import { MatSidenav } from "@angular/material/sidenav";

@Directive({
    selector: "[sideNavToggle]",
    standalone: false
})
export class SideNavToggleDirective implements OnInit, OnDestroy {
  isMobile;
  screenSizeWatcher: Subscription;
  constructor(
    private mediaObserver: MediaObserver,
    @Host() @Self() @Optional() public sideNav: MatSidenav,
  ) {}

  ngOnInit() {
    this.initSideNav();
  }

  ngOnDestroy() {
    if (this.screenSizeWatcher) {
      this.screenSizeWatcher.unsubscribe();
    }
  }

  updateSidenav() {
    const self = this;
    setTimeout(() => {
      self.sideNav.opened = !self.isMobile;
      self.sideNav.mode = self.isMobile ? "over" : "side";
    });
  }
  initSideNav() {
    this.isMobile =
      this.mediaObserver.isActive("xs") || this.mediaObserver.isActive("sm");
    // console.log(this.isMobile)
    this.updateSidenav();
    this.screenSizeWatcher = this.mediaObserver
      .asObservable()
      .subscribe((changes: MediaChange[]) => {
        changes.forEach((change) => {
          this.isMobile = change.mqAlias == "xs" || change.mqAlias == "sm";
        });
        this.updateSidenav();
      });
  }
}
