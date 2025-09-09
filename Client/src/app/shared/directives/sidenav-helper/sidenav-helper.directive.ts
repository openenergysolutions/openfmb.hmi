// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

import { Directive, OnInit, OnDestroy, HostBinding, Input, HostListener, inject } from "@angular/core";
import { takeUntil } from "rxjs/operators";
import { Subject } from "rxjs";
import { MatchMediaService } from "../../services/match-media.service";
import { SidenavHelperService } from "./sidenav-helper.service";
import { MatSidenav } from "@angular/material/sidenav";
import { MediaObserver } from "ngx-flexible-layout";

@Directive({
    selector: "[sidenavHelper]"
})
export class SidenavHelperDirective implements OnInit, OnDestroy {
  private matchMediaService = inject(MatchMediaService);
  private sidenavHelperService = inject(SidenavHelperService);
  private matSidenav = inject(MatSidenav);
  private mediaObserver = inject(MediaObserver);

  @HostBinding("class.is-open")
  isOpen: boolean;

  @Input()
  sidenavHelper: string;

  @Input()
  isOpenBreakpoint: string;

  private unsubscribeAll: Subject<any>;

  constructor() {
    // Set the default value
    this.isOpen = true;

    this.unsubscribeAll = new Subject();
  }

  ngOnInit(): void {
    this.sidenavHelperService.setSidenav(this.sidenavHelper, this.matSidenav);

    if (this.mediaObserver.isActive(this.isOpenBreakpoint)) {
      this.isOpen = true;
      this.matSidenav.mode = "side";
      this.matSidenav.toggle(true);
    } else {
      this.isOpen = false;
      this.matSidenav.mode = "over";
      this.matSidenav.toggle(false);
    }

    this.matchMediaService.onMediaChange
      .pipe(takeUntil(this.unsubscribeAll))
      .subscribe(() => {
        if (this.mediaObserver.isActive(this.isOpenBreakpoint)) {
          this.isOpen = true;
          this.matSidenav.mode = "side";
          this.matSidenav.toggle(true);
        } else {
          this.isOpen = false;
          this.matSidenav.mode = "over";
          this.matSidenav.toggle(false);
        }
      });
  }

  ngOnDestroy(): void {
    this.unsubscribeAll.complete();
  }
}

@Directive({
    selector: "[sidenavToggler]"
})
export class SidenavTogglerDirective {
  private sidenavHelperService = inject(SidenavHelperService);

  @Input()
  public sidenavToggler: any;

  @HostListener("click")
  onClick() {
    this.sidenavHelperService.getSidenav(this.sidenavToggler).toggle();
  }
}
