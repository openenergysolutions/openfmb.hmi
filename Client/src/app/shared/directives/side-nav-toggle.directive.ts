// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Directive, OnDestroy, OnInit, inject } from "@angular/core";
import { MediaChange, MediaObserver } from "ngx-flexible-layout";
import { Subscription } from "rxjs";
import { MatSidenav } from "@angular/material/sidenav";

@Directive({
    selector: "[sideNavToggle]"
})
export class SideNavToggleDirective implements OnInit, OnDestroy {
  private mediaObserver = inject(MediaObserver);
  sideNav = inject(MatSidenav, { host: true, self: true, optional: true });

  isMobile;
  screenSizeWatcher: Subscription;

  ngOnInit() {
    this.initSideNav();
  }

  ngOnDestroy() {
    if (this.screenSizeWatcher) {
      this.screenSizeWatcher.unsubscribe();
    }
  }

  updateSidenav() {    
    setTimeout(() => {
      this.sideNav.opened = !this.isMobile;
      this.sideNav.mode = this.isMobile ? "over" : "side";
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
